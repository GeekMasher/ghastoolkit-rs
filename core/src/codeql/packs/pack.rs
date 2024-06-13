//! CodeQL Pack
use std::{collections::HashMap, fmt::Display, path::PathBuf};

use crate::GHASError;

/// CodeQL Pack
#[derive(Debug, Clone, Default)]
pub struct CodeQLPack {
    /// Path
    path: PathBuf,
    /// Pack Yaml
    pack: PackYaml,
    /// Pack Type
    pack_type: CodeQLPackType,
    /// Pack Lock
    pack_lock: Option<PackYamlLock>,
}

impl CodeQLPack {
    /// Create a new CodeQL Pack
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path: PathBuf = path.into();

        if path.exists() {
            Self::load(path.clone()).unwrap_or_default()
        } else {
            Self {
                path,
                pack: PackYaml::default(),
                pack_type: CodeQLPackType::Queries,
                pack_lock: None,
            }
        }
    }
    /// Get the pack name
    pub fn get_name(&self) -> String {
        self.pack.name.clone()
    }
    /// Get the root path of the CodeQL Pack
    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    /// Get the pack version
    pub fn get_version(&self) -> Option<String> {
        self.pack.version.clone()
    }
    /// Get the list of dependencies for the pack.
    ///
    /// If the Pack Lock is available, it will return the dependencies from the lock file.
    /// Otherwise, it will return the dependencies from the pack file.
    pub fn get_dependencies(&self) -> HashMap<String, String> {
        if let Some(pack_lock) = &self.pack_lock {
            pack_lock
                .dependencies
                .iter()
                .map(|(key, value)| (key.clone(), value.version.clone()))
                .collect()
        } else {
            self.pack.dependencies.clone().unwrap_or_default()
        }
    }
    /// Get the pack type
    pub fn get_type(&self) -> CodeQLPackType {
        self.pack_type.clone()
    }

    /// Load a QLPack from a path (root directory or qlpack.yml file)
    pub fn load(path: impl Into<PathBuf>) -> Result<Self, GHASError> {
        // Path is the directory
        let mut path: PathBuf = path.into();

        if !path.exists() {
            return Err(GHASError::CodeQLPackError(path.display().to_string()));
        }
        if path.is_file() {
            // TODO: Is this the best way to handle this?
            path = path.parent().unwrap().to_path_buf();
        }

        let qlpack_path = path.join("qlpack.yml");
        let qlpack_lock_path = path.join("codeql-pack.lock.yml");

        let pack: PackYaml = match serde_yaml::from_reader(std::fs::File::open(&qlpack_path)?) {
            Ok(p) => p,
            Err(e) => return Err(GHASError::YamlError(e)),
        };
        let pack_type = Self::get_pack_type(&pack);

        let pack_lock: Option<PackYamlLock> = match std::fs::File::open(qlpack_lock_path) {
            Ok(f) => match serde_yaml::from_reader(f) {
                Ok(p) => Some(p),
                Err(e) => return Err(GHASError::YamlError(e)),
            },
            Err(_) => None,
        };

        Ok(Self {
            path,
            pack,
            pack_type,
            pack_lock,
        })
    }

    /// Based on the loaded YAML, determine the pack type
    fn get_pack_type(pack_yaml: &PackYaml) -> CodeQLPackType {
        if let Some(library) = pack_yaml.library {
            if library {
                return CodeQLPackType::Library;
            } else if pack_yaml.data_extensions.is_some() {
                return CodeQLPackType::Models;
            }
        } else if pack_yaml.tests.is_some() {
            return CodeQLPackType::Testing;
        } else if pack_yaml.data_extensions.is_some() {
            return CodeQLPackType::Models;
        }

        CodeQLPackType::Queries
    }
}

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
#[derive(Debug, Clone, Default, serde::Deserialize)]
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
#[derive(Debug, Clone, Default, serde::Deserialize)]
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
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct PackYamlLockDependency {
    /// Version
    pub version: String,
}
