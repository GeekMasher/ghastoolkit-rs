#![forbid(unsafe_code)]
#![allow(dead_code)]

pub mod codescanning;
pub mod errors;
pub mod octokit;
pub mod utils;

/// GitHub API errors
pub use errors::GHASError;
/// GitHub API client
pub use octokit::github::GitHub;
pub use octokit::repository::Repository;
