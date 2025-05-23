//! Extractor Module

pub mod models;

pub use models::CodeQLExtractor;

/// Build Mode
///
/// https://docs.github.com/en/enterprise-cloud@latest/code-security/code-scanning/creating-an-advanced-setup-for-code-scanning/codeql-code-scanning-for-compiled-languages#codeql-build-modes
#[derive(Debug, Clone)]
pub enum BuildMode {
    /// build-mode: none or buildless
    None,
    /// Auto Build
    AutoBuild,
    /// Manual Build
    Manual,
}

impl From<&str> for BuildMode {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "none" | "buildless" => BuildMode::None,
            "autobuild" => BuildMode::AutoBuild,
            "manual" => BuildMode::Manual,
            _ => BuildMode::None,
        }
    }
}

impl From<String> for BuildMode {
    fn from(value: String) -> Self {
        BuildMode::from(value.as_str())
    }
}
