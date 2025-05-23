//! # CodeQL Databases

use log::debug;
use std::path::PathBuf;
use walkdir::WalkDir;

use super::CodeQLLanguage;
use crate::{GHASError, GitHub};
use crate::{Repository, codeql::database::CodeQLDatabase};

/// A list of CodeQL databases
#[derive(Debug, Clone)]
pub struct CodeQLDatabases {
    /// The Base path for the databases
    path: PathBuf,
    /// The list of databases
    databases: Vec<CodeQLDatabase>,
}

impl Iterator for CodeQLDatabases {
    type Item = CodeQLDatabase;

    fn next(&mut self) -> Option<Self::Item> {
        self.databases.pop()
    }
}

impl CodeQLDatabases {
    /// Create a new list of databases
    pub fn new() -> Self {
        Self {
            path: CodeQLDatabases::default_path(),
            databases: Vec::new(),
        }
    }

    /// Set the root path where the databases are stored
    pub fn set_path(&mut self, path: impl Into<PathBuf>) {
        self.path = path.into();
    }

    /// Get all of the loaded databases
    pub fn databases(&self) -> Vec<CodeQLDatabase> {
        self.databases.clone()
    }

    /// Add a database to the list
    pub fn add(&mut self, database: CodeQLDatabase) {
        self.databases.push(database);
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.databases.is_empty()
    }

    /// Get the number of databases in the list
    pub fn len(&self) -> usize {
        self.databases.len()
    }

    /// Dowload a database from the GitHub API
    pub(crate) async fn download_database(
        output: &PathBuf,
        repository: &Repository,
        github: &GitHub,
        language: &CodeQLLanguage,
    ) -> Result<CodeQLDatabase, GHASError> {
        if github.is_enterprise_server() {
            return Err(GHASError::CodeQLDatabaseError(
                "CodeQL database download is not supported on GitHub Enterprise Server".to_string(),
            ));
        }

        let dbpath = output.join("codeql-database.zip");
        let route = format!(
            "{base}repos/{owner}/{repo}/code-scanning/codeql/databases/{language}",
            base = github.base(),
            owner = repository.owner(),
            repo = repository.name(),
            language = language.language()
        );
        log::debug!("Route: {}", route);

        let client = reqwest::Client::new();
        let mut request = client
            .get(route)
            .header(
                http::header::ACCEPT,
                http::header::HeaderValue::from_str("application/zip")?,
            )
            .header(
                http::header::USER_AGENT,
                http::header::HeaderValue::from_str("ghastoolkit")?,
            );

        if let Some(token) = github.token() {
            request = request.header(http::header::AUTHORIZATION, format!("Bearer {}", token));
        }

        let data = request.send().await?.bytes().await?;

        tokio::fs::write(&dbpath, data).await?;
        log::debug!("Database archive downloaded to {}", dbpath.display());

        if !dbpath.exists() {
            return Err(crate::GHASError::CodeQLDatabaseError(format!(
                "Database not found at: {}",
                output.display()
            )));
        }

        log::debug!("Unzipping CodeQL database to {}", output.display());
        Self::unzip_codeql_database(&dbpath, &output)?;

        let mut db = CodeQLDatabase::load(output)?;
        db.set_repository(repository);

        Ok(db)
    }

    /// Download a database and store it in the CodeQL Databases path
    pub async fn download(
        &mut self,
        repository: &Repository,
        github: &GitHub,
    ) -> Result<Vec<CodeQLDatabase>, GHASError> {
        let mut databases = Vec::new();
        let database_list = github
            .code_scanning(repository)
            .list_codeql_databases()
            .await?;

        for dbitem in database_list {
            let language = CodeQLLanguage::from(dbitem.language);

            let path = Self::default_db_path(&self.path, repository, language.language());
            if !path.exists() {
                debug!("Creating database path: {}", path.display());
                tokio::fs::create_dir_all(&path).await?;
            }
            debug!("Downloading database to: {}", path.display());

            let db = Self::download_database(&path, repository, github, &language).await?;
            log::debug!("Database: {db:?}");

            self.add(db.clone());
            databases.push(db);
        }

        Ok(databases)
    }

    /// Download a database for a specific language
    pub async fn download_language(
        &mut self,
        repository: &Repository,
        github: &GitHub,
        language: impl Into<CodeQLLanguage>,
    ) -> Result<CodeQLDatabase, GHASError> {
        let language = language.into();

        let path = Self::default_db_path(&self.path, repository, language.language());
        if !path.exists() {
            debug!("Creating database path: {}", path.display());
            tokio::fs::create_dir_all(&path).await?;
        }
        log::debug!("Downloading database to: {}", path.display());
        let db = Self::download_database(&path, repository, github, &language).await?;
        log::debug!("Database: {db:?}");

        self.add(db.clone());

        Ok(db)
    }

    /// Unzip the CodeQL database
    fn unzip_codeql_database(zip: &PathBuf, output: &PathBuf) -> Result<(), GHASError> {
        log::debug!("Unzipping CodeQL database to {}", output.display());
        let file = std::fs::File::open(zip)?;
        let mut archive = zip::ZipArchive::new(file)?;
        archive.extract(output)?;

        Ok(())
    }

    /// Create a default path for the database to be stored
    pub(crate) fn default_db_path(
        base: &PathBuf,
        repo: &Repository,
        language: impl Into<String>,
    ) -> PathBuf {
        base.join(repo.owner())
            .join(repo.name())
            .join(language.into())
    }

    /// Get the default path for CodeQL databases
    pub fn default_path() -> PathBuf {
        // Get env var CODEQL_DATABASES
        match std::env::var("CODEQL_DATABASES") {
            Ok(p) => PathBuf::from(p),
            Err(_) => {
                // Get HOME directory
                match std::env::var("HOME") {
                    Ok(p) => {
                        let mut base = PathBuf::from(p);
                        base.push(".codeql");
                        base.push("databases");
                        base
                    }
                    Err(_) => PathBuf::from("/tmp/codeql"),
                }
            }
        }
    }

    /// Get the default path for CodeQL results
    pub fn default_results() -> PathBuf {
        match std::env::var("CODEQL_RESULTS") {
            Ok(p) => PathBuf::from(p),
            Err(_) => match std::env::var("HOME") {
                Ok(p) => {
                    let mut base = PathBuf::from(p);
                    base.push(".codeql");
                    base.push("results");
                    base
                }
                Err(_) => PathBuf::from("/tmp/codeql"),
            },
        }
    }

    /// Walk directory to find all CodeQL databases.
    pub fn load(path: impl Into<PathBuf>) -> CodeQLDatabases {
        let path = path.into();
        debug!("Loading databases from: {}", path.display());

        let mut databases = CodeQLDatabases::new();
        databases.path = path.clone();

        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name() == "codeql-database.yml")
            .for_each(|path| {
                let database = CodeQLDatabase::from(path.path());
                databases.add(database);
            });

        databases
    }
}

impl From<String> for CodeQLDatabases {
    fn from(path: String) -> Self {
        CodeQLDatabases::load(path)
    }
}

impl From<&str> for CodeQLDatabases {
    fn from(path: &str) -> Self {
        CodeQLDatabases::load(path.to_string())
    }
}

impl From<PathBuf> for CodeQLDatabases {
    fn from(path: PathBuf) -> Self {
        CodeQLDatabases::load(path.to_str().expect("Invalid path").to_string())
    }
}

impl Default for CodeQLDatabases {
    fn default() -> Self {
        let path = CodeQLDatabases::default_path()
            .to_str()
            .expect("Invalid path")
            .to_string();
        CodeQLDatabases::load(format!("{}/**/codeql-database.yml", path))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::CodeQLDatabases;

    #[test]
    fn test_default_codeql_path() {
        let home_path = match std::env::var("HOME") {
            Ok(p) => {
                let mut path = PathBuf::from(p);
                path.push(".codeql");
                path.push("databases");
                path
            }
            Err(_) => PathBuf::from("/tmp/codeql"),
        };
        let path = CodeQLDatabases::default_path();

        assert_eq!(path, home_path);
    }
}
