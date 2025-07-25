//! # CodeQL Queries

use std::path::PathBuf;

use crate::{GHASError, codeql::languages::CODEQL_LANGUAGES};

/// A collection of CodeQL Queries
/// scope/name@range:path
#[derive(Debug, Default, Clone)]
pub struct CodeQLQueries {
    pub(crate) scope: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) range: Option<String>,
    pub(crate) path: Option<PathBuf>,
}

impl CodeQLQueries {
    /// Parse a string into a CodeQLQueries instance
    pub fn parse(value: impl Into<String>) -> Result<Self, GHASError> {
        let value = value.into();
        if value.is_empty() {
            return Err(GHASError::CodeQLPackError(
                "CodeQLQueries cannot be empty".to_string(),
            ));
        }

        // Absolute or relative path
        if value.starts_with('/') || value.starts_with('.') {
            Ok(Self {
                path: Some(PathBuf::from(value)),
                ..Default::default()
            })
        } else {
            let mut scope = None;
            let mut name = None;
            let mut range = None;
            let mut path = None;

            if let Some((scp, nm)) = value.split_once('/') {
                scope = Some(scp.to_string());

                match nm.split_once('@') {
                    Some((n, rng)) => {
                        name = Some(n.to_string());
                        match rng.split_once(':') {
                            Some((r, p)) => {
                                range = Some(r.to_string());
                                path = Some(PathBuf::from(p));
                            }
                            None => {
                                range = Some(rng.to_string());
                            }
                        }
                    }
                    None => {
                        name = Some(nm.to_string());
                    }
                }
            } else if CODEQL_LANGUAGES
                .iter()
                .find(|lang| lang.0 == value)
                .is_some()
            {
                return Ok(Self::language_default(&value));
            }

            Ok(Self {
                scope,
                name,
                range,
                path,
            })
        }
    }

    /// Create new CodeQL Queries from language
    pub fn language_default(language: &str) -> Self {
        Self {
            scope: Some("codeql".to_string()),
            name: Some(format!("{language}-queries")),
            ..Default::default()
        }
    }

    /// Name of the query
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// Get the scope/namespace of the query
    pub fn scope(&self) -> Option<String> {
        self.scope.clone()
    }

    /// Get the range/version of the query
    pub fn range(&self) -> Option<String> {
        self.range.clone()
    }

    /// Get the suite path
    pub fn suite(&self) -> Option<String> {
        if let Some(path) = &self.path {
            return Some(path.display().to_string());
        }
        None
    }

    /// Set a suite path
    pub fn set_path(&mut self, path: impl Into<PathBuf>) {
        self.path = Some(path.into());
    }
}

impl ToString for CodeQLQueries {
    fn to_string(&self) -> String {
        let mut query = String::new();

        // Pack mode
        if let Some(scope) = &self.scope {
            query += scope;
        }
        if let Some(name) = &self.name {
            query += "/";
            query += name;
        }

        // Range
        if let Some(range) = &self.range {
            query += "@";
            query += range;
        }

        // Path
        if let Some(path) = &self.path {
            if query.is_empty() {
                query = path.to_str().unwrap().to_string();
            } else {
                query += ":";
                query += path.to_str().unwrap();
            }
        }

        query
    }
}

impl From<&str> for CodeQLQueries {
    fn from(value: &str) -> Self {
        Self::parse(value).unwrap_or_else(|_| {
            log::error!("Failed to parse CodeQLQueries from '{}'", value);
            Self::default()
        })
    }
}

impl From<String> for CodeQLQueries {
    fn from(value: String) -> Self {
        Self::parse(&value).unwrap_or_else(|_| {
            log::error!("Failed to parse CodeQLQueries from '{}'", value);
            Self::default()
        })
    }
}

impl From<&String> for CodeQLQueries {
    fn from(value: &String) -> Self {
        Self::parse(value).unwrap_or_else(|_| {
            log::error!("Failed to parse CodeQLQueries from '{}'", value);
            Self::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::codeql::database::queries::CodeQLQueries;

    #[test]
    fn test_pack() {
        let queries = CodeQLQueries::parse("codeql/python-queries")
            .expect("Failed to parse CodeQLQueries from string");

        assert_eq!(queries.scope, Some("codeql".to_string()));
        assert_eq!(queries.name, Some("python-queries".to_string()));
        assert_eq!(queries.range, None);
        assert_eq!(queries.path, None);
    }

    #[test]
    fn test_language_default() {
        let queries = CodeQLQueries::language_default("python");
        assert_eq!(queries.scope, Some("codeql".to_string()));
        assert_eq!(queries.name, Some("python-queries".to_string()));
        assert_eq!(queries.range, None);
        assert_eq!(queries.path, None);
    }

    #[test]
    fn test_string() {
        let query = CodeQLQueries {
            scope: Some("codeql".to_string()),
            name: Some("python-queries".to_string()),
            range: Some("0.9.0".to_string()),
            path: Some(PathBuf::from("codeql-suites/python-code-scanning.qls")),
        };

        assert_eq!(
            query.to_string(),
            "codeql/python-queries@0.9.0:codeql-suites/python-code-scanning.qls"
        );
    }

    #[test]
    fn test_pack_range() {
        let queries = CodeQLQueries::from("codeql/python-queries@0.9.0");
        assert_eq!(queries.scope, Some("codeql".to_string()));
        assert_eq!(queries.name, Some("python-queries".to_string()));
        assert_eq!(queries.range, Some("0.9.0".to_string()));
        assert_eq!(queries.path, None);
    }

    #[test]
    fn test_full() {
        let queries = "codeql/python-queries@0.9.0:codeql-suites/python-code-scanning.qls";
        let codeql_queries = CodeQLQueries::from(queries);

        assert_eq!(codeql_queries.scope, Some(String::from("codeql")));
        assert_eq!(codeql_queries.name, Some(String::from("python-queries")));
        assert_eq!(codeql_queries.range, Some(String::from("0.9.0")));
        assert_eq!(
            codeql_queries.path,
            Some(PathBuf::from("codeql-suites/python-code-scanning.qls"))
        );
    }
}
