//! # CodeQL Builder
//!
//! This module provides a builder for the CodeQL CLI.

use std::path::PathBuf;

use crate::GHASError;

use super::CodeQL;

/// CodeQL Builder to make it easier to create a new CodeQL instance
#[derive(Debug, Clone, Default)]
pub struct CodeQLBuilder {
    path: Option<PathBuf>,

    threads: usize,
    ram: usize,

    search_paths: Vec<PathBuf>,
    additional_packs: Vec<String>,
    showoutput: bool,
}

impl CodeQLBuilder {
    /// Set the path to the CodeQL CLI
    ///
    /// ```rust
    /// use ghastoolkit::codeql::cli::CodeQL;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let codeql = CodeQL::init()
    ///     .path("/path/to/codeql")
    ///     .build()
    ///     .await
    ///     .expect("Failed to create CodeQL instance");
    /// # }
    /// ```
    pub fn path(mut self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        self.path = Some(path);
        self
    }

    /// Set manually the threads for CodeQL
    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    /// Set manually the ram for CodeQL
    pub fn ram(mut self, ram: usize) -> Self {
        self.ram = ram;
        self
    }

    /// Add additional packs to the CodeQL CLI
    pub fn additional_packs(mut self, path: String) -> Self {
        self.additional_packs.push(path);
        self
    }

    /// Add a search path to the CodeQL CLI
    ///
    /// ```rust
    /// use ghastoolkit::codeql::cli::CodeQL;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let codeql = CodeQL::init()
    ///     .search_path("/path/to/codeql")
    ///     .build()
    ///     .await
    ///     .expect("Failed to create CodeQL instance");
    /// # }
    /// ```
    pub fn search_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.search_paths.push(path.into());
        self
    }

    /// Set the show output flag for the CodeQL CLI
    pub fn show_output(mut self, show: bool) -> Self {
        self.showoutput = show;
        self
    }

    /// Build the CodeQL instance
    pub async fn build(&self) -> Result<CodeQL, GHASError> {
        let path: PathBuf = match self.path {
            Some(ref p) => p.clone(),
            None => match CodeQL::find_codeql().await {
                Some(p) => p,
                None => PathBuf::new(),
            },
        };
        log::debug!("CodeQL CLI path: {:?}", path);

        let version: Option<String> = CodeQL::get_version(&path).await.ok();

        Ok(CodeQL {
            version,
            path,
            threads: self.threads,
            ram: self.ram.into(),
            additional_packs: self.additional_packs.clone(),
            search_path: self.search_paths.clone(),
            showoutput: self.showoutput,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_codeql_builder() {
        let codeql = CodeQLBuilder::default()
            .path("/path/to/codeql")
            .threads(4)
            .ram(8)
            .additional_packs("my-pack".to_string())
            .search_path("/path/to/search")
            .show_output(true)
            .build()
            .await
            .unwrap();

        assert_eq!(codeql.path, PathBuf::from("/path/to/codeql"));
        assert_eq!(codeql.threads, 4);
        assert_eq!(codeql.ram, Some(8));
        assert_eq!(codeql.additional_packs, vec!["my-pack".to_string()]);
        assert_eq!(codeql.search_path, vec![PathBuf::from("/path/to/search")]);
        assert_eq!(codeql.showoutput, true);
    }
}
