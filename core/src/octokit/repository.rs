//! # Repository
//!
//! **Example:**
//!
//! ```rust
//! use ghastoolkit::Repository;
//!
//! let repo = Repository::new("geekmasher".to_string(), "ghastoolkit-rs".to_string());
//!
//! ```
use git2::Repository as GitRepository;
use std::{fmt::Display, path::PathBuf};

use log::debug;
use regex::Regex;

use crate::errors::GHASError;

/// GitHub Repository
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Repository {
    /// Owner of the repository (organization or user)
    owner: String,
    /// Name of the repository
    name: String,
    /// Full reference (e.g. refs/heads/main)
    reference: Option<String>,
    /// Branch name (e.g. main)
    branch: Option<String>,

    /// Path to a file or directory relative to the repository root
    path: PathBuf,

    /// Repository root path
    root: PathBuf,
}

impl Repository {
    /// Create a new Repository instance with owner (organization or user) and repo name
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use ghastoolkit::Repository;
    ///
    /// let repo = Repository::new("geekmasher", "ghastoolkit-rs");
    ///
    /// # assert_eq!(repo.owner(), "geekmasher");
    /// # assert_eq!(repo.name(), "ghastoolkit-rs");
    /// ```
    pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            name: repo.into(),
            ..Default::default()
        }
    }

    /// Initialize a new Repository instance with a builder pattern
    ///
    /// # Example
    /// ```rust
    /// use ghastoolkit::Repository;
    ///
    /// let repo = Repository::init()
    ///     .owner("geekmasher")
    ///     .name("ghastoolkit-rs")
    ///     .build();
    /// println!("{:?}", repo);
    /// ```
    pub fn init() -> RepositoryBuilder {
        RepositoryBuilder::default()
    }

    /// Get the Repository owner
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Get the Repository name
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Get the Repository reference
    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }

    /// Get the Repository branch
    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }

    /// Get file or directory relative to the repository root
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Get full path to file or directory relative to the repository root
    pub fn fullpath(&self) -> PathBuf {
        self.root.join(&self.path)
    }

    /// Get root path of the repository
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    /// Set the Repository root path
    pub fn set_root(&mut self, root: PathBuf) {
        self.root = root;
    }

    /// Get the Git SHA of the repository
    pub fn gitsha(&self) -> Option<String> {
        if self.root.exists() {
            // PathBuf to str
            if let Some(path) = self.path.to_str() {
                match GitRepository::open(path) {
                    Ok(repo) => {
                        debug!("Repository found: {:?}", repo.path());
                        // TODO(geekmasher): Handle errors
                        return Some(repo.head().unwrap().target().unwrap().to_string());
                    }
                    Err(e) => {
                        debug!("Failed to open repository: {:?}", e);
                        return None;
                    }
                }
            }
            debug!("Failed to convert PathBuf to str");
            return None;
        }
        debug!("Repository root does not exist");
        None
    }

    /// Parse and return a Repository instance from a repository reference
    ///
    /// # Samples:
    ///
    /// - `geekmasher/ghastoolkit-rs`
    /// - `geekmasher/ghastoolkit-rs@main`
    /// - `geekmasher/ghastoolkit-rs:src/main.rs`
    /// - `geekmasher/ghastoolkit-rs:src/main.rs@main`
    ///
    /// # Example
    ///
    /// ```rust
    /// use ghastoolkit::Repository;
    ///
    /// let repo = Repository::parse("geekmasher/ghastoolkit-rs")
    ///     .expect("Failed to parse repository reference");
    ///
    /// println!("{}", repo);
    /// ```
    ///
    pub fn parse(reporef: &str) -> Result<Repository, GHASError> {
        let mut repository = Repository::default();

        // regex match check
        let re = Regex::new(
            r"^[a-zA-Z0-9-_\.]+/[a-zA-Z0-9-_\.]+((:|/)[a-zA-Z0-9-_/\.]+)?(@[a-zA-Z0-9-_/]+)?$",
        )?;

        re.is_match(reporef).then(|| {
            let mut current = reporef.to_string();
            // parse the repository reference
            match current.split_once('@') {
                Some((repo, branch)) => {
                    repository.branch = Some(branch.to_string());
                    repository.reference = Some(format!("refs/heads/{}", branch));

                    current = repo.to_string();
                }
                _ => {
                    debug!("No reference found in repository reference");
                }
            }
            // TODO(geekmasher): Support for `:` in the repository reference

            let blocks = current.split('/').collect::<Vec<&str>>();
            for (i, block) in blocks.iter().enumerate() {
                match i {
                    0 => repository.owner = block.to_string(),
                    1 => repository.name = block.to_string(),
                    _ => repository.path.push(block),
                }
            }
        });

        Ok(repository)
    }
}

impl Display for Repository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(branch) = &self.branch {
            write!(f, "{}/{}@{}", self.owner, self.name, branch)
        } else {
            write!(f, "{}/{}", self.owner, self.name)
        }
    }
}

impl TryFrom<&str> for Repository {
    type Error = GHASError;

    fn try_from(reporef: &str) -> Result<Self, Self::Error> {
        Repository::parse(reporef)
    }
}

/// Repository Builder pattern
#[derive(Debug, Default, Clone)]
pub struct RepositoryBuilder {
    owner: String,
    name: String,
    reference: Option<String>,
    branch: Option<String>,
    path: PathBuf,
    root: PathBuf,
}

impl RepositoryBuilder {
    /// Set the Repository owner
    pub fn owner(&mut self, owner: &str) -> &mut Self {
        self.owner = owner.to_string();
        self
    }

    /// Set the Repository name
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    /// Set the Repository owner/name
    pub fn repo(&mut self, repo: &str) -> &mut Self {
        if let Some((owner, name)) = repo.split_once('/') {
            self.owner = owner.to_string();
            self.name = name.to_string();
        }
        self
    }

    /// Set the Repository reference
    pub fn reference(&mut self, reference: &str) -> &mut Self {
        self.reference = Some(reference.to_string());
        if let Some((_, branch)) = reference.split_once("heads/") {
            self.branch = Some(branch.to_string());
        }
        self
    }

    /// Set the Repository branch
    pub fn branch(&mut self, branch: &str) -> &mut Self {
        if !branch.is_empty() {
            self.branch = Some(branch.to_string());
            self.reference = Some(format!("refs/heads/{}", branch));
        }
        self
    }

    /// Set the Repository path
    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = PathBuf::from(path);
        self
    }

    /// Set the Repository root source path
    pub fn root(&mut self, root: &str) -> &mut Self {
        self.root = PathBuf::from(root);
        self
    }

    /// Build the Repository
    pub fn build(&self) -> Result<Repository, GHASError> {
        Ok(Repository {
            owner: self.owner.clone(),
            name: self.name.clone(),
            reference: self.reference.clone(),
            branch: self.branch.clone(),
            path: self.path.clone(),
            root: self.root.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_from() {
        let repository = Repository::try_from("owner/repo@main").unwrap();
        assert_eq!(repository.owner, "owner");
        assert_eq!(repository.name, "repo");
        assert_eq!(repository.branch, Some("main".to_string()));

        let repository = Repository::try_from("owner/repo/path/to/file@main").unwrap();
        assert_eq!(repository.owner, "owner");
        assert_eq!(repository.name, "repo");
        assert_eq!(repository.path, PathBuf::from("path/to/file"));
        assert_eq!(repository.branch, Some("main".to_string()));
    }
}
