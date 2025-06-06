//! # Code Scanning module
//!
//! This is used to interact with GitHub's Code Scanning API

use octocrab::Octocrab;

use crate::Repository;

/// GitHub Code Scanning API
pub mod api;
pub mod configuration;
/// GitHub Code Scanning Models
pub mod models;

/// Code Scanning Handler
#[derive(Debug, Clone)]
pub struct CodeScanningHandler<'octo> {
    pub(crate) crab: &'octo Octocrab,
    pub(crate) repository: &'octo Repository,
}
