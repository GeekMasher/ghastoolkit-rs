//! CodeQL Pack
use std::{collections::HashMap, fmt::Display, path::PathBuf};

use crate::codeql::CodeQLLanguage;
use crate::{CodeQLPacks, GHASError};

/// CodeQL Pack
#[derive(Debug, Clone, Default)]
pub struct CodeQLPack {
    /// Name
    name: String,
    /// Owner/Namespace
    namespace: String,
    /// Version
    version: Option<String>,

    /// Path
    path: PathBuf,
    /// Pack Yaml
    pack: Option<PackYaml>,
    /// Pack Type
    pack_type: Option<CodeQLPackType>,
    /// Pack Lock
    pack_lock: Option<PackYamlLock>,
}

impl CodeQLPack {
    /// Create a new CodeQL Pack
    pub fn new(pack: impl Into<String>) -> Self {
        let pack = pack.into();
        Self::try_from(pack.as_str()).unwrap_or_default()
    }
    /// Get the pack name
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// Get the pack namespace
    pub fn namespace(&self) -> String {
        self.namespace.clone()
    }
    /// Get full name (namespace/name)
    pub fn full_name(&self) -> String {
        if let Some(version) = &self.version {
            return format!("{}/{}@{}", self.namespace, self.name, version);
        } else {
            format!("{}/{}", self.namespace, self.name)
        }
    }

    /// Get the root path of the CodeQL Pack
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
    /// Get the pack version
    pub fn version(&self) -> Option<String> {
        if let Some(version) = &self.version {
            return Some(version.clone());
        } else if let Some(pack) = &self.pack {
            return pack.version.clone();
        }
        None
    }

    /// Get the pack language based on the extractor or extension targets
    pub fn language(&self) -> Option<CodeQLLanguage> {
        if let Some(pack) = &self.pack {
            if let Some(extractor) = &pack.extractor {
                return Some(CodeQLLanguage::from(extractor.as_str()));
            } else if let Some(targets) = &pack.extension_targets {
                if let Some((_, lang)) = targets.iter().next() {
                    return Some(CodeQLLanguage::from(lang.as_str()));
                }
            }
        }
        None
    }

    /// Get the list of dependencies for the pack.
    ///
    /// If the Pack Lock is available, it will return the dependencies from the lock file.
    /// Otherwise, it will return the dependencies from the pack file.
    pub fn dependencies(&self) -> HashMap<String, String> {
        if let Some(pack_lock) = &self.pack_lock {
            pack_lock
                .dependencies
                .iter()
                .map(|(key, value)| (key.clone(), value.version.clone()))
                .collect()
        } else if let Some(pack) = &self.pack {
            pack.dependencies.clone().unwrap_or_default()
        } else {
            HashMap::new()
        }
    }
    /// Get the pack type
    pub fn pack_type(&self) -> CodeQLPackType {
        self.pack_type.clone().unwrap_or_default()
    }

    /// Download a CodeQL Pack using its name (namespace/name[@version])
    ///
    /// ```bash
    /// codeql pack download <name>
    /// ```
    #[cfg(feature = "async")]
    pub async fn download(&self, codeql: &crate::CodeQL) -> Result<(), GHASError> {
        log::debug!("Downloading CodeQL Pack: {}", self.full_name());
        codeql
            .run(vec!["pack", "download", self.full_name().as_str()])
            .await?;
        Ok(())
    }

    /// Download a CodeQL Pack using its name (namespace/name[@version])
    pub async fn download_pack(
        codeql: &crate::CodeQL,
        name: impl Into<String>,
    ) -> Result<Self, GHASError> {
        let name = name.into();
        log::debug!("Downloading CodeQL Pack: {name}");
        let pack = CodeQLPack::try_from(name.clone())?;
        pack.download(codeql).await?;

        Ok(pack)
    }

    /// Install the CodeQL Pack Dependencies
    ///
    /// ```bash
    /// codeql pack install <path>
    /// ```
    #[cfg(feature = "async")]
    pub async fn install(&self, codeql: &crate::CodeQL) -> Result<(), GHASError> {
        codeql
            .run(vec!["pack", "install", self.path().to_str().unwrap()])
            .await
            .map(|_| ())
    }

    /// Upgrade CodeQL Pack Dependencies
    #[cfg(feature = "async")]
    pub async fn upgrade(&self, codeql: &crate::CodeQL) -> Result<(), GHASError> {
        codeql
            .run(vec!["pack", "upgrade", self.path().to_str().unwrap()])
            .await
            .map(|_| ())
    }

    /// Publish the CodeQL Pack
    ///
    /// ```bash
    /// codeql pack publish <path>
    /// ```
    #[cfg(feature = "async")]
    pub async fn publish(
        &self,
        codeql: &crate::CodeQL,
        token: impl Into<String>,
    ) -> Result<(), GHASError> {
        Ok(tokio::process::Command::new(codeql.path())
            .env("CODEQL_REGISTRIES_AUTH", token.into())
            .args(vec!["pack", "publish", self.path().to_str().unwrap()])
            .output()
            .await
            .map(|_| ())?)
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

        if !qlpack_path.exists() {
            return Err(GHASError::CodeQLPackError(
                "qlpack.yml file does not exist".to_string(),
            ));
        }

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
        let (namespace, name) = pack.name.split_once('/').unwrap_or_default();

        Ok(Self {
            name: name.to_string(),
            namespace: namespace.to_string(),
            version: pack.version.clone(),
            path,
            pack: Some(pack),
            pack_type: Some(pack_type),
            pack_lock,
        })
    }

    /// Load a CodeQL Pack from the CodeQL Packages Directory
    fn load_package(
        name: impl Into<String>,
        namespace: impl Into<String>,
        version: Option<String>,
    ) -> Result<Self, GHASError> {
        let name = name.into();
        let namespace = namespace.into();
        let version_num = if let Some(ref version) = version {
            version.clone()
        } else {
            "**".to_string()
        };

        let path = CodeQLPacks::codeql_packages_path()
            .join(&namespace)
            .join(&name)
            .join(version_num);

        if path.join("qlpack.yml").exists() {
            log::debug!("Loading pack from path: {}", path.display());
            return Self::load(path);
        }

        // Multiple versions of the same pack can exist in the same directory
        // Find the last / newest version
        let entries = glob::glob(&path.display().to_string())?;
        log::trace!("Entries: {:?}", entries);

        if let Some(Ok(entry)) = entries.last() {
            if entry.exists() {
                return Self::load(entry.clone());
            }
        }

        // If the path does not exist, return a CodeQL Pack with the name, namespace, and version
        Ok(Self {
            name,
            namespace,
            version: version.clone(),
            path,
            ..Default::default()
        })
    }

    /// Based on the loaded YAML, determine the pack type
    fn get_pack_type(pack_yaml: &PackYaml) -> CodeQLPackType {
        if let Some(library) = pack_yaml.library {
            if library && pack_yaml.data_extensions.is_some() {
                return CodeQLPackType::Models;
            } else if library {
                return CodeQLPackType::Library;
            }
        } else if pack_yaml.tests.is_some() {
            return CodeQLPackType::Testing;
        } else if pack_yaml.data_extensions.is_some() {
            return CodeQLPackType::Models;
        }

        CodeQLPackType::Queries
    }
}

impl Display for CodeQLPack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(version) = self.version() {
            write!(f, "{} ({}) - v{}", self.name(), self.pack_type(), version)
        } else {
            write!(f, "{} ({})", self.name(), self.pack_type(),)
        }
    }
}

impl TryFrom<&str> for CodeQLPack {
    type Error = GHASError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let path = PathBuf::from(value);

        if path.exists() {
            Self::load(path.clone())
        } else {
            let (namespace, name) = value.split_once('/').unwrap_or_default();

            if let Some((name, version)) = name.split_once('@') {
                Self::load_package(name, namespace, Some(version.to_string()))
            } else {
                Self::load_package(name, namespace, None)
            }
        }
    }
}

impl TryFrom<String> for CodeQLPack {
    type Error = GHASError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<PathBuf> for CodeQLPack {
    type Error = GHASError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::load(value)
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

#[cfg(test)]
mod tests {
    use anyhow::Context;

    use super::*;

    fn qlpacks() -> PathBuf {
        // Examples
        let mut path = std::env::current_dir().unwrap();
        path.push("../examples/codeql-packs/java/src");
        path.canonicalize().unwrap()
    }

    #[test]
    fn test_codeql_pack() {
        let pack = CodeQLPack::try_from("codeql/python-queries").unwrap();

        assert_eq!(pack.name(), "python-queries");
        assert_eq!(pack.namespace(), "codeql");
        assert_eq!(pack.version(), None);

        let pack = CodeQLPack::try_from("codeql/python-queries@1.0.0").unwrap();

        assert_eq!(pack.name(), "python-queries");
        assert_eq!(pack.namespace(), "codeql");
        assert_eq!(pack.version(), Some("1.0.0".to_string()));
    }

    #[test]
    fn test_codeql_pack_path() {
        let path = qlpacks();
        assert!(path.exists());

        let pack = CodeQLPack::try_from(path.clone())
            .context(format!("Failed to load pack from path: {}", path.display()))
            .unwrap();

        assert_eq!(pack.path(), path);
        assert_eq!(pack.name(), "codeql-java");
        assert_eq!(pack.namespace(), "geekmasher");
        assert_eq!(pack.version(), Some("1.0.0".to_string()));
        assert_eq!(pack.pack_type(), CodeQLPackType::Queries);
    }
}
