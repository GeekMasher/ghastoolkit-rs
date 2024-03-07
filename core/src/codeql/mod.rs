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
//!
//! You can also use the builder pattern to create a new CodeQL CLI instance:
//!
//! ```rust
//! use ghastoolkit::codeql::CodeQL;
//!
//! let codeql = CodeQL::init()
//!     .path(String::from("/path/to/codeql"))
//!     .threads(4)
//!     .ram(8000)
//!     .build()
//!     .expect("Failed to create CodeQL instance");
//! ```
//!
//! ## CodeQL Database
//!
//! If you want to create and analyze a CodeQL database, you can use the `CodeQLDatabase` struct:
//!
//! ```no_run
//! use ghastoolkit::codeql::{CodeQL, CodeQLDatabase};
//!
//! let codeql = CodeQL::default();
//!
//! // Create a new CodeQL database
//! let database = CodeQLDatabase::init()
//!     .name("ghastoolkit")
//!     .language("python")
//!     .source(String::from("/path/to/source"))
//!     .build()
//!     .expect("Failed to create CodeQL database");
//!
//! println!("Database :: {}", database);
//!
//! // Create a new CodeQL database
//! codeql.database(&database)
//!     .overwrite()
//!     .create()
//!     .expect("Failed to create CodeQL database");
//!
//! let results = codeql.database(&database)
//!     .analyze()
//!     .expect("Failed to analyze CodeQL database");
//!```

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
