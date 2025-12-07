use crate::{models::WtxError, services::RepositoryService};

pub fn execute(url: &str) -> Result<(), WtxError> {
    let mut repository_service = RepositoryService::new()?;
    repository_service.register(url)?;
    Ok(())
}
