use std::path::PathBuf;

use log::debug;
use walkdir::WalkDir;

use crate::codeql::database::CodeQLDatabase;

/// A list of CodeQL databases
#[derive(Debug, Clone)]
pub struct CodeQLDatabases {
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
            databases: Vec::new(),
        }
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
    pub fn load(path: String) -> CodeQLDatabases {
        debug!("Loading databases from: {}", path);
        let mut databases = CodeQLDatabases::new();

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
