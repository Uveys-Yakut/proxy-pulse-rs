use crate::core::domain::Error as DomainError;

#[derive(Debug, Clone)]
pub enum Error {
    ExternalError(String),
    OperationFailed(String),
    Domain(DomainError),
    Unexpected,
}

impl From<DomainError> for Error {
    fn from(e: DomainError) -> Self {
        Self::Domain(e)
    }
}
