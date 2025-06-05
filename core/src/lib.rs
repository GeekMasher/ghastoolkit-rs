#![forbid(unsafe_code)]
#![allow(dead_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod codeql;
pub mod codescanning;
pub mod errors;
pub mod octokit;
pub mod secretscanning;
pub mod supplychain;
pub mod utils;

pub use errors::GHASError;

pub use octokit::github::GitHub;
pub use octokit::repository::Repository;

// CodeQL
pub use codeql::extractors::{BuildMode, CodeQLExtractor};
pub use codeql::packs::{CodeQLPack, CodeQLPackType, CodeQLPacks};
pub use codeql::{CodeQL, CodeQLDatabase, CodeQLDatabases};

// Supply Chain
pub use supplychain::{Dependencies, Dependency, License, Licenses};

// Utilities
pub use utils::sarif::Sarif;

#[doc(hidden)]
#[allow(unused_imports, missing_docs)]
pub mod prelude {
    pub use crate::codeql::extractors::{BuildMode, CodeQLExtractor};
    pub use crate::codeql::packs::{CodeQLPack, CodeQLPackType, CodeQLPacks};
    pub use crate::codeql::{CodeQL, CodeQLDatabase, CodeQLDatabases};
    pub use crate::errors::GHASError;
    pub use crate::octokit::github::GitHub;
    pub use crate::octokit::repository::Repository;
    pub use crate::supplychain::{Dependencies, Dependency, License, Licenses};
}
