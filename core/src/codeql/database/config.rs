//! # CodeQL Database Configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::GHASError;

/// CodeQL Database Configuration which is stored in the database directory
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeQLDatabaseConfig {
    /// Source Location Prefix
    #[serde(rename = "sourceLocationPrefix")]
    pub source_location_prefix: Option<String>,
    /// Database primary language (e.g. cpp, java, python, etc.)
    #[serde(rename = "primaryLanguage")]
    pub primary_language: String,
    /// Database baseline lines of code
    #[serde(rename = "baselineLinesOfCode")]
    pub baseline_lines_of_code: usize,
    /// Unicode Newlines
    #[serde(rename = "unicodeNewlines")]
    pub unicode_newlines: bool,
    /// Database column kind
    #[serde(rename = "columnKind")]
    pub column_kind: String,
    /// Database creation metadata
    #[serde(rename = "creationMetadata")]
    pub creation_metadata: Option<CodeQLDatabaseConfigMetadata>,
    /// Build Mode
    #[serde(rename = "buildMode")]
    pub build_mode: Option<String>,
    /// Finalized
    #[serde(default, rename = "finalised")]
    pub finalised: bool,
}

impl CodeQLDatabaseConfig {
    /// Read, parse, and return a CodeQL Database Configuration from the provided path
    pub fn read(path: &PathBuf) -> Result<CodeQLDatabaseConfig, GHASError> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let config: CodeQLDatabaseConfig = serde_yaml::from_reader(reader)?;
        Ok(config)
    }
}

/// CodeQL Database Configuration Metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeQLDatabaseConfigMetadata {
    /// Database SHA
    pub sha: Option<String>,
    /// Database CLI Version
    #[serde(rename = "cliVersion")]
    pub cli_version: String,
    /// Database Creation Time
    #[serde(rename = "creationTime")]
    pub creation_time: chrono::DateTime<chrono::Utc>,
}
