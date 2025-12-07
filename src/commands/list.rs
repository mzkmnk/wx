use crate::{
    models::{Repository, WtxError},
    services::RepositoryService,
};

pub fn execute() -> Result<Vec<Repository>, WtxError> {
    let repository_service = RepositoryService::new()?;
    repository_service.list()
}
