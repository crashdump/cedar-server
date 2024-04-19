#[allow(clippy::all)]
use crate::api::proto::management_service_server::ManagementService;
use crate::api::proto::{
    CreatePolicyRequest, CreatePolicyResponse,
    GetPolicyRequest, GetPolicyResponse,
    ListPoliciesRequest, ListPoliciesResponse,
    DeletePolicyRequest, DeletePolicyResponse,
    PolicyDefinition,
};
use crate::storage::repository;
use crate::storage::model::{NewPolicy, Policy, Store};

use futures::future::join_all;
use tonic::{Request, Response, Status};
use tracing::info;
use uuid::Uuid;

pub struct ManagementServiceImpl {
    pub repository: repository::Repository,
}

#[tonic::async_trait]
impl ManagementService for ManagementServiceImpl{
    async fn create_policy(
        &self,
        request: Request<CreatePolicyRequest>,
    ) -> Result<Response<CreatePolicyResponse>, Status> {
        info!("Got a `CreatePolicy` request from {:?}", request.remote_addr());

        let req = request.into_inner();

        let store_id: Uuid = Uuid::parse_str(&req.policy_store_id)
            .map_err(|e| Status::new(tonic::Code::InvalidArgument, e.to_string()))?;

        let conn = self.repository.get_connection()?;

        let store = Store::get(conn?, &store_id)?;

        NewPolicy::new(&store, &req.description, &req.statement)
            .create(conn?)
            .and_then(|policy| {
                Ok(CreatePolicyResponse {
                    policy_store_id: store.id.to_string(),
                    policy: Some(PolicyDefinition {
                        id: policy.id.to_string(),
                        description: policy?.description,
                        statement: policy?.statement,
                    })
                })
            })
            .respond()
    }

    async fn get_policy(
        &self,
        request: Request<GetPolicyRequest>,
    ) -> Result<Response<GetPolicyResponse>, Status> {
        info!("Got a `GetPolicy` request from {:?}", request.remote_addr());

        let req = request.into_inner();

        let store_id: Uuid = Uuid::parse_str(&req.policy_store_id)
            .map_err(|e| Status::new(tonic::Code::InvalidArgument, e.to_string()))?;

        let policy_id: Uuid = Uuid::parse_str(&req.policy_id)
            .map_err(|e| Status::new(tonic::Code::InvalidArgument, e.to_string()))?;

        let conn = self.repository.get_connection()?;

        let store = Store::get(conn?, &store_id)?;

        Policy::get(conn?, &store, &policy_id)
            .and_then(|policy| {
                Ok(CreatePolicyResponse {
                    policy_store_id: store.id.to_string(),
                    policy: Some(PolicyDefinition {
                        id: policy.id.to_string(),
                        description: policy?.description,
                        statement: policy?.statement,
                    })
                })
            })
            .respond()
    }

    async fn list_policies(
        &self,
        request: Request<ListPoliciesRequest>,
    ) -> Result<Response<ListPoliciesResponse>, Status> {
        info!("Got a `ListPolicies` request from {:?}", request.remote_addr());

        let req = request.into_inner();
        let conn = self.repository.get_connection()?;

        let store_id: Uuid = Uuid::parse_str(&req.policy_store_id)
            .map_err(|e| Status::new(tonic::Code::InvalidArgument, e.to_string()))?;

        let store = Store::get(conn?, &store_id)?;

        Policy::list(conn?, &store)
            .and_then(|policies| {
                let proto_policies = policies.iter().map(|policy| {
                    PolicyDefinition {
                        id: policy.id,
                        description: "".to_string(),
                        statement: "".to_string(),
                    }
                }).collect();
                Ok(ListPoliciesResponse {
                    policy_store_id: store.id.to_string(),
                    policies: proto_policies,
                })
            })
            .respond();
    }

    async fn delete_policy(
        &self,
        request: Request<DeletePolicyRequest>,
    ) -> Result<Response<DeletePolicyResponse>, Status> {
        info!("Got a `DeletePolicy` request from {:?}", request.remote_addr());

        let req = request.into_inner();

        let store_id: Uuid = Uuid::parse_str(&req.policy_store_id)
            .map_err(|e| Status::new(tonic::Code::InvalidArgument, e.to_string()))?;

        let policy_id: Uuid = Uuid::parse_str(&req.policy_id)
            .map_err(|e| Status::new(tonic::Code::InvalidArgument, e.to_string()))?;


        let _ = self.repository.get_policy(store_id, policy_id).await;

        Ok(
            Response::new(DeletePolicyResponse {
                policy_store_id: store_id.into(),
            })
        )
    }
}
