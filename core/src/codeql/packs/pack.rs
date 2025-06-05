//! CodeQL Pack
use std::{collections::HashMap, fmt::Display, path::PathBuf};

use crate::GHASError;
use crate::codeql::CodeQLLanguage;
use crate::codeql::database::queries::CodeQLQueries;

use super::models::CodeQLPackType;
use super::{PackYaml, PackYamlLock};

/// CodeQL Pack
#[derive(Debug, Clone, Default)]
pub struct CodeQLPack {
    /// CodeQL Queries reference
    pub(crate) queries: CodeQLQueries,

    /// Path
    pub(crate) path: PathBuf,
    /// Pack Yaml
    pub(crate) pack: Option<PackYaml>,
    /// Pack Type
    pub(crate) pack_type: Option<CodeQLPackType>,
    /// Pack Lock
    pub(crate) pack_lock: Option<PackYamlLock>,
}

impl CodeQLPack {
    /// Create a new CodeQL Pack
    pub fn new(pack: impl Into<String>) -> Self {
        let pack = pack.into();
        if let Ok(path) = PathBuf::from(&pack).canonicalize() {
            return Self::load(path).unwrap_or_default();
        } else {
            Self::load_remote_pack(pack).unwrap_or_default()
        }
    }

    /// Get the pack name
    pub fn name(&self) -> String {
        self.queries.name().unwrap_or_default()
    }

    /// Get the pack namespace
    pub fn namespace(&self) -> String {
        self.queries.scope().unwrap_or_else(|| "codeql".to_string())
    }

    /// Get full name (namespace/name[@version][:suite])
    pub fn full_name(&self) -> String {
        let mut full_name = format!("{}/{}", self.namespace(), self.name());
        if let Some(version) = self.queries.range() {
            full_name.push_str(&format!("@{}", version));
        }
        if let Some(suite) = self.queries.suite() {
            full_name.push_str(&format!(":{}", suite));
        }

        full_name
    }

    /// Get the root path of the CodeQL Pack
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    /// Get the pack version
    pub fn version(&self) -> Option<String> {
        if let Some(version) = &self.queries.range() {
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

    /// Get the default suite file for the pack
    pub fn suite(&self) -> Option<String> {
        if let Some(pack) = &self.pack {
            return pack.default_suite_file.clone();
        }
        None
    }

    /// Check if the pack is installed
    pub async fn is_installed(&self) -> bool {
        self.path.exists()
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

    /// Based on the loaded YAML, determine the pack type
    pub(crate) fn get_pack_type(pack_yaml: &PackYaml) -> CodeQLPackType {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codeql_pack_display() {
        let pack = CodeQLPack::new("codeql/javascript-queries@1.0.0");
        assert_eq!(pack.to_string(), "javascript-queries (Queries) - v1.0.0");
    }

    #[test]
    fn test_codeql_pack_full_name() {
        let pack = CodeQLPack::new("codeql/javascript-queries@1.0.0");
        assert_eq!(pack.full_name(), "codeql/javascript-queries@1.0.0");
    }

    #[test]
    fn test_codeql_pack_namespace() {
        let pack = CodeQLPack::new("codeql/javascript-queries");
        assert_eq!(pack.namespace(), "codeql");
        assert_eq!(pack.name(), "javascript-queries");

        assert_eq!(pack.full_name(), "codeql/javascript-queries");
    }
}
