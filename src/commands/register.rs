use crate::{models::WxError, services::RepositoryService};

pub fn execute(url: &str) -> Result<(), WxError> {
    let mut repository_service = RepositoryService::new()?;
    repository_service.register(url)?;
    Ok(())
}
