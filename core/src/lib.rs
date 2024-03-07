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
//!   - [ ] Secret Scanning
//!
//! ## Usage
//!
//! ```rust
//! use ghastoolkit::{GitHub, Repository};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Initialize GitHub using environment variables or github.com
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

/// CodeQL module is used to interact with CodeQL CLI and databases
pub mod codeql;
/// Code Scanning module is used to interact with GitHub's Code Scanning API
pub mod codescanning;
/// GHASToolkit errors module contains all the errors that can be thrown by the library
pub mod errors;
/// GitHub Octokit client for interacting with GitHub's API endpoints
pub mod octokit;
/// GHASToolkit utils module contains all the utility functions and helpers
pub mod utils;

/// GitHub API errors
pub use errors::GHASError;
/// GitHub API client
pub use octokit::github::GitHub;
pub use octokit::repository::Repository;

// CodeQL
pub use codeql::CodeQL;
pub use codeql::CodeQLDatabase;
pub use codeql::CodeQLDatabases;
