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
pub use supplychain::{Dependencies, Dependency};
