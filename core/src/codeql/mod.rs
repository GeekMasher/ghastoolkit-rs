//! # CodeQL
//!
//! This module contains a simple interface to work with CodeQL CLI and databases in Rust.
//!
//! ## Usage
//!
//! ```rust
//! use ghastoolkit::codeql::CodeQL;
//!
//! let codeql = CodeQL::default();
//!
//! println!("CodeQL :: {}", codeql);
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
