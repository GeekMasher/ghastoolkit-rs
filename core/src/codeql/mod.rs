//! # CodeQL
//!
//! This module contains a simple interface to work with CodeQL CLI and databases in Rust.
//!
//! ## Usage
//!
//! ```no_run
//! use ghastoolkit::codeql::{CodeQL, CodeQLDatabase, CodeQLDatabases};
//!
//! // Setup a default CodeQL CLI
//! let codeql = CodeQL::default();
//! println!("CodeQL :: {}", codeql);
//!
//! // Get all CodeQL databases from the default path
//! let databases = CodeQLDatabases::default();
//!
//! for database in databases {
//!    println!("Database :: {}", database);
//! }
//! ```
/// This module contains the codeql struct and its methods
pub mod cli;
/// This module contains the codeql database struct and its methods
pub mod database;
/// This module contains the codeql databases struct and its methods
pub mod databases;
/// This module contains the codeql language struct and its methods
pub mod languages;

pub use cli::CodeQL;
pub use database::CodeQLDatabase;
pub use databases::CodeQLDatabases;
pub use languages::CodeQLLanguage;
