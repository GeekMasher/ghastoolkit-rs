//! # GHASToolkit Errors
//!
//! This module contains all the errors that can be thrown by the library
use octocrab::Error as OctocrabError;
use regex::Error as RegexError;

/// GitHub Advanced Security Toolkit Error
#[derive(thiserror::Error, Debug)]
pub enum GHASError {
    /// Repository Reference Error
    #[error("RepositoryReferenceError: {0}")]
    RepositoryReferenceError(String),

    /// CodeQL Error
    #[error("CodeQLError: {0}")]
    CodeQLError(String),

    /// CodeQL Database Error
    #[error("CodeQLDatabaseError: {0}")]
    CodeQLDatabaseError(String),

    /// CodeQL Pack Error
    #[error("CodeQLPackError: {0}")]
    CodeQLPackError(String),

    /// Octocrab Error (octocrab::Error)
    #[error("OctocrabError: {0}")]
    OctocrabError(#[from] OctocrabError),

    /// GHActions Error
    #[cfg(feature = "toolcache")]
    #[error("GHActionsError: {0}")]
    GHActionsError(#[from] ghactions::ActionsError),

    /// Regex Error (regex::Error)
    #[error("RegexError: {0}")]
    RegexError(#[from] RegexError),

    /// Io Error (std::io::Error)
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),

    /// Serde Error (serde_json::Error)
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),

    /// Yaml Error (serde_yaml::Error)
    #[error("YamlError: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// Url Error (url::ParseError)
    #[error("UrlError: {0}")]
    UrlError(#[from] url::ParseError),

    /// Git Errors (git2::Error)
    #[error("GitErrors: {0}")]
    GitErrors(#[from] git2::Error),

    /// Zip Error (zip::result::ZipError)
    #[error("ZipError: {0}")]
    ZipError(#[from] zip::result::ZipError),

    /// Reqwest Error (reqwest::Error)
    #[error("ReqwestError: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// Http Error (http::Error)
    #[error("HttpError: {0}")]
    HttpInvalidHeader(#[from] http::header::InvalidHeaderValue),

    /// Walkdir Error (walkdir::Error)
    #[error("WalkdirError: {0}")]
    WalkdirError(#[from] walkdir::Error),

    /// Glob Error (glob::PatternError)
    #[error("GlobError: {0}")]
    GlobError(#[from] glob::PatternError),

    /// Unknown Error
    #[error("UnknownError: {0}")]
    UnknownError(String),
}
