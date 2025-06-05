//! # CodeQLPack Loader
//!
//! This module provides functionality to load CodeQL Packs from various sources, including local directories and remote repositories.
use std::path::PathBuf;

use crate::codeql::database::queries::CodeQLQueries;
use crate::codeql::packs::{PackYaml, PackYamlLock};
use crate::{CodeQLPacks, GHASError};

/// # CodeQLPack Loader
use super::pack::CodeQLPack;

impl CodeQLPack {
    /// Load a QLPack from a path (root directory or qlpack.yml file)
    pub fn load(path: impl Into<PathBuf>) -> Result<Self, GHASError> {
        // Path is the directory
        let mut path: PathBuf = path.into();
        log::debug!("Loading CodeQL Pack from path: {}", path.display());

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

        let queries = CodeQLQueries::from(&pack.name.clone());
        Ok(Self {
            queries,
            path,
            pack: Some(pack),
            pack_type: Some(pack_type),
            pack_lock,
        })
    }

    pub(crate) fn load_remote_pack(remote: impl Into<String>) -> Result<Self, GHASError> {
        let queries = CodeQLQueries::from(remote.into());

        // Load the pack from the CodeQL Packages Directory
        if let Ok(pack) = Self::load_package(
            queries.name().unwrap_or_default(),
            queries.scope().unwrap_or_default(),
            queries.range(),
        ) {
            Ok(pack)
        } else {
            Ok(Self {
                queries,
                ..Default::default()
            })
        }
    }

    /// Load a CodeQL Pack from the CodeQL Packages Directory
    ///
    /// It will try to load the pack from the specified namespace, name, and optinal version.
    pub(crate) fn load_package(
        name: impl Into<String>,
        namespace: impl Into<String>,
        version: Option<String>,
    ) -> Result<Self, GHASError> {
        let name = name.into();
        let namespace = namespace.into();
        let queries = CodeQLQueries {
            name: Some(name.clone()),
            scope: Some(namespace.clone()),
            range: version.clone(),
            path: None,
        };
        let version_num = if let Some(ref version) = version {
            version.clone()
        } else {
            "**".to_string()
        };

        let path = CodeQLPacks::codeql_packages_path()
            .join(&namespace)
            .join(&name)
            .join(version_num);
        log::debug!("Loading CodeQL Pack from path: {}", path.display());

        let qlpack_path = path.join("qlpack.yml");

        if qlpack_path.exists() {
            log::debug!("Loading pack from path: {}", path.display());
            return Self::load(path);
        }

        // Multiple versions of the same pack can exist in the same directory
        // Find the last / newest version
        if let Ok(entries) = glob::glob(&qlpack_path.display().to_string()) {
            log::trace!("Entries: {:?}", entries);

            if let Some(Ok(entry)) = entries.last() {
                if entry.exists() {
                    return Self::load(entry.clone());
                }
            }
        }

        // If the path does not exist, return a CodeQL Pack with the name, namespace, and version
        Ok(Self {
            queries,
            path: PathBuf::new(),
            ..Default::default()
        })
    }
}

impl TryFrom<&str> for CodeQLPack {
    type Error = GHASError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(path) = PathBuf::from(value).canonicalize() {
            log::debug!("Loading CodeQL Pack from path: {}", path.display());

            if path.exists() {
                return Self::load(path.clone());
            }
        }
        Self::load_remote_pack(value)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CodeQLPackType;
    use anyhow::Context;

    fn qlpacks() -> PathBuf {
        // Examples
        let mut path = std::env::current_dir().unwrap();
        path.push("../examples/codeql-packs/java/src");
        path.canonicalize().unwrap()
    }

    #[test]
    fn test_codeql_pack() {
        let pack = CodeQLPack::try_from("codeql/python-queries")
            .expect("Failed to create CodeQLPack from string");

        assert_eq!(pack.name(), "python-queries");
        assert_eq!(pack.namespace(), "codeql");

        let pack = CodeQLPack::try_from("codeql/python-queries@1.0.0")
            .expect("Failed to create CodeQLPack from string with version");

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
