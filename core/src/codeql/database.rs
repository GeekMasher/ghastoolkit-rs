//! # CodeQL Database
//!
//! This module defines the CodeQL Database.
//! This structure is used to interact with CodeQL databases.
//! It provides methods to validate, build, and handle databases.

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use log::debug;

use crate::{
    CodeQLDatabases, GHASError, GitHub, Repository,
    codeql::{CodeQLLanguage, database::config::CodeQLDatabaseConfig},
};

pub mod config;
pub mod handler;
pub mod queries;

/// CodeQL Database
#[derive(Debug, Clone, Default)]
pub struct CodeQLDatabase {
    name: String,
    /// The path to the database
    path: PathBuf,
    /// The language of the database
    language: CodeQLLanguage,
    /// The source root of the database
    source: Option<PathBuf>,
    /// Repository the database is associated with
    repository: Option<Repository>,
    /// Configuration
    config: Option<CodeQLDatabaseConfig>,
}

impl CodeQLDatabase {
    /// Create a new CodeQLDatabase
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize a new CodeQLDatabaseBuilder
    pub fn init() -> CodeQLDatabaseBuilder {
        CodeQLDatabaseBuilder::default()
    }

    /// Get the database name
    pub fn name(&self) -> String {
        if self.name.is_empty() {
            // Repo
            if let Some(ref repo) = self.repository {
                return repo.name().to_string();
            } else if self.path.exists() {
                // TODO: This only works for the default path
                let base = CodeQLDatabases::default_path();
                let path = self.path.strip_prefix(&base).unwrap_or(&self.path);
                let components = path.components().collect::<Vec<_>>();
                if components.len() == 1 {
                    // If there is only one component, it is the name
                    return components[0].as_os_str().to_string_lossy().to_string();
                } else if components.len() > 1 {
                    // If there are more than one component, it is the name
                    return components[1].as_os_str().to_string_lossy().to_string();
                }
            } else if let Some(source) = &self.source {
                // TODO(geekmasher): This is a bit of a hack, but it works for now
                return source
                    .clone()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
            }
        }
        self.name.clone()
    }

    /// Get the database language
    pub fn language(&self) -> &str {
        self.language.language()
    }

    /// Get the repository the database is associated with
    pub fn repository(&self) -> Option<&Repository> {
        self.repository.as_ref()
    }

    /// Set the repository the database is associated with
    pub(crate) fn set_repository(&mut self, repository: &Repository) {
        self.repository = Some(repository.clone());
        self.name = repository.name().to_string();
    }

    /// Get the database path (root directory)
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Get the path to the CodeQL Database configuration file
    pub fn configuration_path(&self) -> PathBuf {
        let mut path = self.path.clone();
        path.push("codeql-database.yml");
        path
    }

    /// Creation time of the database
    pub fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        if let Some(config) = &self.config {
            if let Some(metadata) = &config.creation_metadata {
                return Some(metadata.creation_time);
            }
        }
        None
    }

    /// Check if the database is valid (configuration file exists)
    pub fn validate(&self) -> bool {
        let path = self.configuration_path();
        path.exists()
    }

    /// Get the version of the CodeQL CLI used to create the database
    /// If the version is not available, it will return "0.0.0"
    pub fn version(&self) -> String {
        if let Some(config) = &self.config {
            if let Some(metadata) = &config.creation_metadata {
                return metadata.cli_version.clone();
            }
        }
        String::from("0.0.0")
    }

    /// Get the creation time of the database
    pub fn creation_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        if let Some(config) = &self.config {
            if let Some(metadata) = &config.creation_metadata {
                return Some(metadata.creation_time);
            }
        }
        None
    }

    /// Get the number of lines of code in the database
    pub fn lines_of_code(&self) -> usize {
        if let Some(config) = &self.config {
            return config.baseline_lines_of_code;
        }
        0
    }

    /// Reload the database configuration
    pub fn reload(&mut self) -> Result<(), GHASError> {
        debug!("Reloading CodeQL Database Configuration");
        if self.validate() {
            let config = CodeQLDatabaseConfig::read(&self.configuration_path())?;
            self.config = Some(config);
            Ok(())
        } else {
            Err(GHASError::CodeQLDatabaseError(
                "Invalid CodeQL Database".to_string(),
            ))
        }
    }

    /// Load a database from a directory
    pub fn load(path: impl Into<PathBuf>) -> Result<CodeQLDatabase, GHASError> {
        let path = path.into();
        if !path.exists() {
            return Err(GHASError::CodeQLDatabaseError(
                "Could not find codeql-database.yml".to_string(),
            ));
        }

        // If the path is a file, we need to pop it to get the directory
        if path.is_file() && path.ends_with("codeql-database.yml") {
            debug!("Loading CodeQL Database from: {}", path.display());
            CodeQLDatabase::load_database_config(&path)
        } else {
            log::debug!("Finding CodeQL Database from: {}", path.display());
            // If the path is a directory, we need to find the configuration file
            let mut dbroot = PathBuf::new();

            // Look for `codeql-database.yml` in the directory recursively
            for entry in walkdir::WalkDir::new(&path) {
                let entry = entry?;
                if entry.file_name() == "codeql-database.yml" {
                    dbroot = entry.path().to_path_buf();
                    break;
                }
            }

            log::debug!("Loading CodeQL Database from: {}", dbroot.display());
            CodeQLDatabase::load_database_config(&dbroot)
        }
    }

    /// Download / Fetch database from GitHub
    pub async fn download(
        output: PathBuf,
        repository: &Repository,
        github: &GitHub,
    ) -> Result<Vec<Self>, GHASError> {
        let mut databases = CodeQLDatabases::new();
        databases.set_path(output);
        databases.download(repository, github).await?;
        Ok(databases.databases())
    }

    fn load_database_config(path: &PathBuf) -> Result<CodeQLDatabase, GHASError> {
        if !path.exists() {
            Err(GHASError::CodeQLDatabaseError(
                "Could not find codeql-database.yml".to_string(),
            ))
        } else {
            match CodeQLDatabaseConfig::read(path) {
                Ok(config) => CodeQLDatabase::init()
                    .path(path.parent().unwrap_or(path))
                    .source(config.source_location_prefix.clone().unwrap_or_default())
                    .language(config.primary_language.clone())
                    .config(config.clone())
                    .build(),
                Err(err) => {
                    log::error!("Failed to load database configuration: {}", err);
                    Err(GHASError::CodeQLDatabaseError(
                        "Failed to load database configuration".to_string(),
                    ))
                }
            }
        }
    }
}

impl From<String> for CodeQLDatabase {
    fn from(path: String) -> Self {
        CodeQLDatabase::load(path).expect("Failed to load CodeQL Database")
    }
}

impl From<&str> for CodeQLDatabase {
    fn from(path: &str) -> Self {
        CodeQLDatabase::load(path.to_string()).expect("Failed to load CodeQL Database")
    }
}

impl From<PathBuf> for CodeQLDatabase {
    fn from(path: PathBuf) -> Self {
        CodeQLDatabase::load(path.to_string_lossy().to_string())
            .expect("Failed to load CodeQL Database")
    }
}

impl From<&Path> for CodeQLDatabase {
    fn from(path: &Path) -> Self {
        CodeQLDatabase::load(path.to_string_lossy().to_string())
            .expect("Failed to load CodeQL Database")
    }
}

impl Display for CodeQLDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version = self.version();
        if version.as_str() == "0.0.0" {
            write!(f, "CodeQLDatabase('{}', '{}')", self.name(), self.language)
        } else {
            write!(
                f,
                "CodeQLDatabase('{}', '{}', '{}')",
                self.name(),
                self.language,
                version
            )
        }
    }
}

/// CodeQL Database Builder used for creating a new CodeQLDatabase's using the builder pattern
///
/// # Example
///
/// ```rust
/// use ghastoolkit::codeql::CodeQLDatabase;
///
/// // Using the `init` method to create a new CodeQLDatabaseBuilder
/// let database = CodeQLDatabase::init()
///     .name("test".to_string())
///     .path("/path/to/database".to_string())
///     .language("javascript".to_string())
///     .source("/path/to/source".to_string())
///     .build()
///     .expect("Failed to build database");
///
/// ```
#[derive(Debug, Clone, Default)]
pub struct CodeQLDatabaseBuilder {
    name: String,
    path: Option<PathBuf>,
    language: CodeQLLanguage,
    source: Option<PathBuf>,
    repository: Option<Repository>,
    config: Option<CodeQLDatabaseConfig>,
}

impl CodeQLDatabaseBuilder {
    /// Set the name of the database
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use ghastoolkit::CodeQLDatabase;
    ///
    /// let database = CodeQLDatabase::init()
    ///     .name("test")
    ///     .path("/path/to/database")
    ///     .language("javascript")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the path to the database. If there is an existing database, it will load the configuration
    /// file from the path.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use ghastoolkit::CodeQLDatabase;
    ///
    /// let database = CodeQLDatabase::init()
    ///     .name("test")
    ///     .path("/path/to/database")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn path(mut self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        self.path = Some(path.clone());

        if path.exists() {
            let config_path = if path.is_dir() {
                path.join("codeql-database.yml")
            } else if path.is_file() {
                path.clone()
            } else {
                log::warn!("Unknown path type: {:?}", path);
                return self;
            };

            debug!("Loading database configuration: {:?}", &config_path);

            let config = match CodeQLDatabaseConfig::read(&config_path) {
                Ok(config) => config,
                Err(e) => {
                    debug!("Failed to load database configuration: {}", e);
                    return self;
                }
            };
            debug!("Loaded database configuration: {:?}", &path);

            self.language = CodeQLLanguage::from(config.primary_language);
            if let Some(source) = config.source_location_prefix {
                self.source = Some(PathBuf::from(source));
            }
        }
        self
    }

    /// Set the source root for database creation / mapping
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use ghastoolkit::CodeQLDatabase;
    ///
    /// let database = CodeQLDatabase::init()
    ///     .name("test")
    ///     .source("./src")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn source(mut self, source: impl Into<PathBuf>) -> Self {
        let source = source.into();
        self.source = Some(PathBuf::from(source));
        self
    }

    /// Set the language of the database
    ///
    /// **Examples:**
    ///
    /// ```rust
    /// use ghastoolkit::CodeQLDatabase;
    ///
    /// let database = CodeQLDatabase::init()
    ///     .name("test")
    ///     .language("javascript")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn language(mut self, language: impl Into<CodeQLLanguage>) -> Self {
        self.language = language.into();
        self
    }

    /// Set the repository the database is associated with
    pub fn repository(mut self, repository: &Repository) -> Self {
        self.name = repository.name().to_string();
        self.repository = Some(repository.clone());
        self
    }

    /// Set the configuration for the database
    pub fn config(mut self, config: CodeQLDatabaseConfig) -> Self {
        self.language = CodeQLLanguage::from(config.primary_language.clone());
        self.config = Some(config);
        self
    }

    /// Get the default path for the database
    pub(crate) fn default_path(&self) -> PathBuf {
        let mut path = CodeQLDatabases::default_path();

        if let Some(ref repo) = self.repository {
            path.push(repo.owner());
            path.push(repo.name());
            path.push(self.language.language());
        } else if !self.language.is_secondary() {
            path.push(format!("{}-{}", self.language.language(), self.name));
        } else {
            path.push(self.name.clone());
        }

        path
    }

    /// Build the CodeQLDatabase from the builder
    pub fn build(&self) -> Result<CodeQLDatabase, GHASError> {
        let path = match self.path.clone() {
            Some(p) => p,
            None => self.default_path(),
        };

        let name = if self.name.is_empty() {
            if let Some(ref repo) = self.repository {
                // If a repository is set, use its name
                repo.name().to_string()
            } else if let Some(ref source) = self.source {
                // If the source is set, use it to derive the name
                source
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            } else {
                // Fallback to the path if no name is set
                path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            }
        } else {
            self.name.clone()
        };

        Ok(CodeQLDatabase {
            name,
            path,
            language: self.language.clone(),
            source: self.source.clone(),
            repository: self.repository.clone(),
            config: self.config.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{CodeQLDatabase, Repository};
    use std::path::PathBuf;

    #[test]
    fn test_default_database_path() {
        let base = match std::env::var("HOME") {
            Ok(p) => {
                let mut path = PathBuf::from(p);
                path.push(".codeql");
                path.push("databases");
                path
            }
            Err(_) => PathBuf::from("/tmp/codeql"),
        };

        let mut path = base.clone();
        path.push("python-test-repo");

        let db = CodeQLDatabase::init()
            .name(String::from("test-repo"))
            .language("python".to_string())
            .build()
            .expect("Failed to build database");

        assert_eq!(db.language(), "python");
        assert_eq!(db.path, path);
    }

    #[test]
    fn test_database_name() {
        // Set the name of the database
        let db = CodeQLDatabase::init()
            .name(String::from("test-repo"))
            .language("python".to_string())
            .build()
            .expect("Failed to build database");

        assert_eq!(db.name, "test-repo");
    }

    #[test]
    fn test_database_from_source() {
        let db2 = CodeQLDatabase::init()
            .source(String::from("/tmp/test-repo"))
            .language("python".to_string())
            .build()
            .expect("Failed to build database");
        assert_eq!(db2.name, "test-repo");
    }

    #[test]
    fn test_database_from_repository() {
        let repo = Repository::parse("geekmasher/test-repo").unwrap();
        let db3 = CodeQLDatabase::init()
            .repository(&repo)
            .language("python".to_string())
            .build()
            .expect("Failed to build database");

        assert_eq!(db3.name, "test-repo");
    }
}
