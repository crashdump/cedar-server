use common::context;
use tracing_subscriber;
use uuid::Uuid;

use cedar_server_lib;

mod common;

#[tokio::test]
async fn db_crud_tests() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let _ctx = context::TestContext::new();

    let repository = cedar_server_lib::storage::repository::Repository::new(context::TEST_PG_URL.into());

    let store_id = Uuid::now_v7();

    let test_policy_id = match repository.create_policy(store_id, "foo".into(), "bar".into()).await {
        Ok(got) => {
            assert_eq!("foo", got.description);
            assert_eq!("bar", got.statement);
            got.id
        }
        Err(_) => { panic!() }
    };

    match repository.get_policy(store_id, test_policy_id).await {
        Ok(got) => {
            assert_eq!("foo", got.description);
            assert_eq!("bar", got.statement);
        }
        Err(_) => { panic!() }
    }

    match  repository.list_policies(store_id).await {
        Ok(got) => {
            assert_eq!(1, got.len());
        }
        Err(_) => { panic!() }
    }

    repository.delete_policy(store_id, test_policy_id).
        await.
        expect("delete policy failed");

    match repository.list_policies(store_id).await {
        Ok(got) => {
            assert_eq!(0, got.len());
        }
        Err(_) => { panic!() }
    }

}