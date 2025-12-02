use crate::{
    models::{Repository, WtxError},
    service::repository::RepositoryService,
};

pub fn execute() -> Result<Vec<Repository>, WtxError> {
    let repository_service = RepositoryService::new()?;
    Ok(repository_service.list()?)
}
