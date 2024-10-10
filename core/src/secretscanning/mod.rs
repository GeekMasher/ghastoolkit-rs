//! # Secret Scanning
//!
//! # Example
//!
//! ```no_run
//! # use anyhow::Result;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//!
//! let github = ghastoolkit::GitHub::init()
//!     .owner("geekmasher")
//!     .token("personal_access_token")
//!     .build()
//!     .expect("Failed to initialise GitHub instance");
//!
//! let repo = ghastoolkit::Repository::new("geekmasher", "ghastoolkit-rs");
//!
//! let alerts = github
//!     .secret_scanning(&repo)
//!     .list()
//!     .send()
//!     .await?;
//!
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod secretalerts;
