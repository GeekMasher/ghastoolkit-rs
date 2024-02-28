use octocrab::Error as OctocrabError;
use regex::Error as RegexError;

#[derive(thiserror::Error, Debug)]
pub enum GHASError {
    #[error("RepositoryReferenceError: {0}")]
    RepositoryReferenceError(String),

    #[error("OctocrabError: {0}")]
    OctocrabError(#[from] OctocrabError),

    #[error("RegexError: {0}")]
    RegexError(#[from] RegexError),

    #[error("UrlError: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("GitErrors: {0}")]
    GitErrors(#[from] git2::Error),

    #[error("UnknownError: {0}")]
    UnknownError(String),
}
