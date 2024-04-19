mod common;

use tonic::Request;
use common::context;
use tracing_subscriber;
use uuid::Uuid;

use cedar_server_lib;
use cedar_server_lib::api::proto::{CreatePolicyRequest, GetPolicyRequest, ListPoliciesRequest, DeletePolicyRequest, GetPolicyResponse};
use cedar_server_lib::api::proto::management_service_server::ManagementService;

#[tokio::test]
async fn api_mgmt_tests() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let _ctx = context::TestContext::new();

    let repository = cedar_server_lib::storage::repository::Repository::new(context::TEST_PG_URL.into());
    let api_mgmt = cedar_server_lib::api::management::ManagementServiceImpl {
        repository,
    };

    let store_id = Uuid::now_v7();

    let test_policy_id = match api_mgmt.create_policy(
        Request::new(CreatePolicyRequest{
            policy_store_id: store_id.to_string(),
            description: "foo".to_string(),
            statement: "bar".to_string(),
        })
    ).await {
        Ok(res) => {
            let got = res.into_inner();
            let got_policy = got.policy.unwrap();
            assert_eq!("foo", got_policy.description);
            assert_eq!("bar", got_policy.statement);
            got_policy.id
        }
        Err(_) => { panic!() }
    };

    match api_mgmt.get_policy(
        Request::new(GetPolicyRequest{
            policy_store_id: store_id.to_string(),
            policy_id: test_policy_id.clone(),
        })
    ).await {
        Ok(res) => {
            let got = res.into_inner();
            let got_policy = got.policy.unwrap();
            assert_eq!("foo", got_policy.description);
            assert_eq!("bar", got_policy.statement);
        }
        Err(_) => { panic!() }
    };

    match api_mgmt.list_policies(
        Request::new(ListPoliciesRequest{
            policy_store_id: store_id.to_string(),
        })
    ).await {
        Ok(res) => {
            let got = res.into_inner();
            assert_eq!(1, got.policies.len());
        }
        Err(_) => { panic!() }
    };

    // List all policies again, should be 0.

    api_mgmt.delete_policy(
        Request::new(DeletePolicyRequest{
            policy_store_id: store_id.to_string(),
            policy_id: test_policy_id.clone(),
        })
    ).await.expect("delete policy failed");

    // List all policies again, should be 0.
    match api_mgmt.list_policies(
        Request::new(ListPoliciesRequest{
            policy_store_id: store_id.to_string(),
        })
    ).await {
        Ok(res) => {
            let got = res.into_inner();
            assert_eq!(0, got.policies.len());
        }
        Err(_) => { panic!() }
    };

}
