use dotenvy::dotenv;
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber;

use cedar_server_lib::api::management::ManagementServiceImpl;
use cedar_server_lib::api::proto::management_service_server::ManagementServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let config = Config::init();
    let server_listen = config.server_listen.clone();
    let db_url = config.database_url.clone();

    let repository = cedar_server_lib::storage::repository::Repository::new(db_url);
    repository.run_migrations().await;
    
    let management_api = ManagementServiceImpl { repository };

    let addr = server_listen.parse().unwrap();
    Server::builder()
        .add_service(ManagementServiceServer::new(management_api))
        .serve(addr)
        .await?;

    info!("Server listening on {}", addr);

    Ok(())
}

fn get_env_var(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| panic!("{} must be set", var_name))
}

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub server_listen: String,
    pub database_url: String,
}

impl Config {
    pub(crate) fn init() -> Config {
        dotenv().ok();

        let server_listen = get_env_var("SERVER_LISTEN");
        let database_url = get_env_var("DATABASE_URL");

        Config {
            server_listen,
            database_url,
        }
    }
}
