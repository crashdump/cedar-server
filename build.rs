use tonic_buf_build::{compile_from_buf_workspace, error};
use tonic_build::configure;

fn main() -> Result<(), error::TonicBufBuildError> {
    compile_from_buf_workspace(configure(), None)?;
    Ok(())
}
