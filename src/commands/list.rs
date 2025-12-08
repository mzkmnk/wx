use crate::{
    models::{Repository, WxError},
    services::RepositoryService,
};

pub fn execute() -> Result<Vec<Repository>, WxError> {
    let repository_service = RepositoryService::new()?;
    repository_service.list()
}
