use crate::{models::WxError, services::RepositoryService};

pub fn execute(name: &str) -> Result<(), WxError> {
    let mut repository_service = RepositoryService::new()?;
    repository_service.unregister(name)?;
    Ok(())
}
