use crate::{models::WtxError, services::RepositoryService};

pub fn execute(name: &str) -> Result<(), WtxError> {
    let mut repository_service = RepositoryService::new()?;
    repository_service.unregister(name)?;
    Ok(())
}
