pub mod auth_service;
pub mod v1;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
enum GeneralError {
    #[error("Not found error.")]
    NotFoundError,

    #[error("General error.")]
    GeneralError,
}
