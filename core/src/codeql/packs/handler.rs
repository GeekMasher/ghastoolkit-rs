//! # CodeQL Pack Handler
//!
//! This module provides a handler for managing CodeQL packs, which are collections of queries and databases used in CodeQL analysis.
use super::CodeQLPack;
use crate::{CodeQL, GHASError, codeql::database::queries::CodeQLQueries};

/// CodeQL Database Handler
#[derive(Debug, Clone)]
pub struct CodeQLPackHandler<'db, 'ql> {
    /// Reference to the CodeQL Database
    pack: &'db CodeQLPack,
    /// Reference to the CodeQL instance
    codeql: &'ql CodeQL,
    /// Optional suite name for the pack
    suite: Option<String>,
}

impl<'db, 'ql> CodeQLPackHandler<'db, 'ql> {
    /// Creates a new CodeQLPackHandler with the given pack and CodeQL instance.
    pub fn new(pack: &'db CodeQLPack, codeql: &'ql CodeQL) -> Self {
        Self {
            pack,
            codeql,
            suite: None,
        }
    }

    /// Sets the suite name for the pack handler.
    pub fn suite(mut self, suite: impl Into<String>) -> Self {
        self.suite = match CodeQLQueries::parse(suite.into()) {
            Ok(q) => q.suite(),
            Err(e) => Some(e.to_string()),
        };
        self
    }

    fn get_suite(&self) -> Option<String> {
        if let Some(suite) = &self.suite {
            return Some(suite.clone());
        } else if let Some(suite) = &self.codeql.suite {
            return Some(suite.clone());
        }
        None
    }

    /// Resolves the queries in the pack and returns a list of query names grouped by language.
    ///
    /// The query path / name is returned relative to the pack path.
    pub async fn resolve(&mut self) -> Result<Vec<String>, GHASError> {
        let mut args = vec!["resolve", "queries", "--format=json"];

        let name = if let Some(suite) = &self.get_suite() {
            format!("{}:{}", self.pack.full_name(), suite)
        } else {
            self.pack.full_name()
        };
        args.push(name.as_str());
        log::debug!("Resolving queries for pack: {}", name);
        println!("Resolving queries for pack: {}", name);

        let output = self.codeql.run(args).await?;
        let json: Vec<String> = serde_json::from_str(&output)?;

        let mut pack_path = self.pack.path().display().to_string();
        if !pack_path.ends_with('/') {
            pack_path.push('/');
        }

        // Remove the pack path
        Ok(json
            .iter()
            .map(|query| query.replace(&pack_path, ""))
            .collect())
    }

    /// Downloads the CodeQL pack.
    pub async fn download(&self) -> Result<(), GHASError> {
        log::debug!("Downloading CodeQL Pack: {}", self.pack.full_name());
        self.codeql
            .run(vec!["pack", "download", self.pack.full_name().as_str()])
            .await?;
        Ok(())
    }
}
