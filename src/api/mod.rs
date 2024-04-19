pub mod proto {
    tonic::include_proto!("management.v1");
    tonic::include_proto!("verification.v1");
}

pub mod management;
pub mod verification;
