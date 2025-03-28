//! Extractor Module

pub mod models;

pub use models::CodeQLExtractor;

/// Build Mode
///
/// https://docs.github.com/en/enterprise-cloud@latest/code-security/code-scanning/creating-an-advanced-setup-for-code-scanning/codeql-code-scanning-for-compiled-languages#codeql-build-modes
pub enum BuildMode {
    /// build-mode: none or buildless
    None,
    /// Auto Build
    AutoBuild,
    /// Manual Build
    Manual,
}
