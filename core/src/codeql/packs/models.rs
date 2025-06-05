//! # CodeQL Packs Models
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};

/// CodeQL Pack Type
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodeQLPackType {
    /// CodeQL Library
    Library,
    /// CodeQL Queries
    #[default]
    Queries,
    /// CodeQL Models
    Models,
    /// CodeQL Testing
    Testing,
}

impl Display for CodeQLPackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeQLPackType::Library => write!(f, "Library"),
            CodeQLPackType::Queries => write!(f, "Queries"),
            CodeQLPackType::Models => write!(f, "Models"),
            CodeQLPackType::Testing => write!(f, "Testing"),
        }
    }
}

/// CodeQL Pack Yaml Structure
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PackYaml {
    /// The Pack Name
    pub name: String,
    /// Pack is a Library or not
    pub library: Option<bool>,
    /// The Pack Version
    pub version: Option<String>,
    /// Pack Groups
    pub groups: Option<Vec<String>>,
    /// The Pack Dependencies
    pub dependencies: Option<HashMap<String, String>>,

    /// The Pack Suites
    pub suites: Option<String>,
    /// The Pack Default Suite File
    #[serde(rename = "defaultSuiteFile")]
    pub default_suite_file: Option<String>,

    /// The Pack Extractor name
    pub extractor: Option<String>,

    /// Extension Targets
    #[serde(rename = "extensionTargets")]
    pub extension_targets: Option<HashMap<String, String>>,
    /// Data Extensions
    #[serde(rename = "dataExtensions")]
    pub data_extensions: Option<Vec<String>>,

    /// The Pack Tests Directory
    pub tests: Option<String>,
}

/// CodeQL Pack Lock Yaml Structure
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PackYamlLock {
    /// Lock Version
    #[serde(rename = "lockVersion")]
    pub lock_version: String,
    /// Dependencies
    pub dependencies: HashMap<String, PackYamlLockDependency>,
    /// If the pack is compiled
    pub compiled: bool,
}

/// CodeQL Pack Lock Dependency
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PackYamlLockDependency {
    /// Version
    pub version: String,
}
