//! GitHub Advanced Security (GHAS) Toolkit is a library for interacting with various GitHub's API and features.
//! The main goal of this library is to provide a simple and easy to use interface with these features.
//!
//! ## Features
//!
//! There are a few features that are currently supported by this library:
//!
//! - [x] CodeQL
//!   - [x] CodeQL CLI
//!   - [x] CodeQL Database(s)
//! - [x] GitHub Advanced Security APIs
//!   - [x] Code Scanning
//!   - [x] Secret Scanning
//!
//! ## Usage
//!
//! ```rust
//! use ghastoolkit::{GitHub, Repository};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Initialize GitHub using default environment variables or github.com
//!     let github = GitHub::default();
//!     println!("GitHub :: {}", github);
//!
//!     let repository = Repository::parse("geekmasher/ghastoolkit-rs@main")
//!         .expect("Failed to create Repository");
//!     println!("Repository :: {}", repository);
//! }
//!
#![forbid(unsafe_code)]
#![allow(dead_code)]
#![deny(missing_docs)]
#![doc = include_str!("../../README.md")]

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
