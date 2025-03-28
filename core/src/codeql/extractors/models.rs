//! CodeQL Extractor YAML Model

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// CodeQL Extractor
///
/// ```rust
/// use ghastoolkit::codeql::CodeQLExtractor;
///
/// let extractor = CodeQLExtractor::load(r#"
///     name: "rust"
///     display_name: "Rust"
///     version: 0.1.0
///     column_kind: "utf8"
///     legacy_qltest_extraction: true
///     github_api_languages:
///       - Rust
///     scc_languages:
///       - Rust
///     file_types:
///       - name: rust
///         display_name: Rust
///         extensions:
///           - rs
///
/// "#).unwrap();
///
/// # assert_eq!(extractor.name, "rust");
/// # assert_eq!(extractor.display_name, "Rust");
/// # assert_eq!(extractor.version, "0.1.0");
///
///
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CodeQLExtractor {
    /// Root Directory of the extractor-pack
    #[serde(skip)]
    pub path: PathBuf,
    /// The name of the extractor
    pub name: String,
    /// The display name of the extractor
    pub display_name: String,
    /// The version of the extractor
    pub version: String,
    /// The build modes
    #[serde(default)]
    pub build_modes: Vec<String>,
    /// Column kind
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_kind: Option<String>,
    /// Legacy QLTest extraction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_qltest_extraction: Option<bool>,
    /// GitHub API languages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_api_languages: Option<Vec<String>>,
    /// SCC languages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scc_languages: Option<Vec<String>>,
    /// File types
    pub file_types: Vec<CodeQLExtractorFileType>,
}

impl CodeQLExtractor {
    /// Load CodeQL Extractor from a string
    pub fn load(content: &str) -> Result<Self, crate::errors::GHASError> {
        let extractor: CodeQLExtractor = serde_yaml::from_str(content)?;
        Ok(extractor)
    }
    /// Load CodeQL Extractor from a file
    pub fn load_path(path: impl Into<PathBuf>) -> Result<Self, crate::errors::GHASError> {
        let mut path: PathBuf = path.into();
        if path.is_dir() {
            path = path.join("codeql-extractor.yml");
        }
        log::debug!("Loading CodeQL Extractor from: {}", path.display());
        if !path.exists() {
            return Err(crate::GHASError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            )));
        }
        let content = std::fs::read_to_string(&path)?;
        let mut extractor: CodeQLExtractor = serde_yaml::from_str(&content)?;
        if let Some(parent) = path.parent() {
            extractor.path = parent.to_path_buf();
        }
        Ok(extractor)
    }

    /// Get supported languages for an extractor
    pub fn languages(&self) -> Vec<String> {
        let mut results = Vec::new();
        results.extend(self.github_api_languages.clone().unwrap_or_default());
        results
    }

    /// Get the build modes
    pub fn build_modes(&self) -> Vec<super::BuildMode> {
        self.build_modes
            .iter()
            .map(|mode| match mode.as_str() {
                "none" | "buildless" => super::BuildMode::None,
                "auto" => super::BuildMode::AutoBuild,
                "manual" => super::BuildMode::Manual,
                _ => super::BuildMode::None,
            })
            .collect()
    }
}

/// CodeQL Extractor File Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CodeQLExtractorFileType {
    /// Name
    pub name: String,
    /// Display name
    pub display_name: String,
    /// File Extensions
    pub extensions: Vec<String>,
}
