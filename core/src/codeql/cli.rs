use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use log::debug;

use crate::{
    codeql::{database::handler::CodeQLDatabaseHandler, CodeQLLanguage},
    CodeQLDatabase, GHASError,
};

mod models;

use models::ResolvedLanguages;

/// CodeQL CLI Wrapper to make it easier to run CodeQL commands
#[derive(Debug, Clone)]
pub struct CodeQL {
    /// Path to the CodeQL CLI
    path: PathBuf,
    /// Number of threads to use
    threads: usize,
    /// Amount of RAM to use
    ram: Option<usize>,

    /// The search path for the CodeQL CLI
    search_path: Vec<String>,
    /// Additional packs to use
    additional_packs: Vec<String>,
}

impl CodeQL {
    /// Create a new CodeQL instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize a new CodeQL Builder instance
    pub fn init() -> CodeQLBuilder {
        CodeQLBuilder::default()
    }

    /// Find CodeQL CLI on the system
    pub fn find_codeql() -> Option<PathBuf> {
        if let Some(p) = CodeQL::find_codeql_path() {
            return Some(p);
        }
        // TODO(geekmasher): Add support for GitHub Actions Tool Cache

        None
    }

    fn find_codeql_path() -> Option<PathBuf> {
        debug!("Looking for CodeQL in PATH");
        // Check if CodeQL is in the PATH
        if let Ok(paths) = std::env::var("PATH") {
            for path in paths.split(':') {
                let p = Path::new(path).join("codeql");
                if p.exists() && p.is_file() {
                    return Some(p);
                }
            }
        }
        None
    }

    /// Run a CodeQL command
    pub fn run(&self, args: Vec<&str>) -> Result<String, GHASError> {
        debug!("{:?}", args);
        let mut cmd = std::process::Command::new(&self.path);
        cmd.args(args);

        let output = cmd.output()?;

        if output.status.success() {
            debug!("CodeQL Command Success: {:?}", output.status.to_string());
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(GHASError::CodeQLError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }

    /// Pass a CodeQLDatabase to the CodeQL CLI to return a CodeQLDatabaseHandler.
    /// This handler can be used to run queries and other operations on the database.
    pub fn database<'a>(&'a self, db: &'a CodeQLDatabase) -> CodeQLDatabaseHandler {
        CodeQLDatabaseHandler::new(db, self)
    }

    /// Get the version of the CodeQL CLI
    pub fn version(&self) -> Result<String, GHASError> {
        match self.run(vec!["version", "--format", "terse"]) {
            Ok(v) => Ok(v.trim().to_string()),
            Err(e) => Err(e),
        }
    }

    /// Get the programming languages supported by the CodeQL CLI.
    /// This function will return the primary languages supported by the CodeQL and exclude
    /// any secondary languages (checkout `get_secondary_languages()`).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ghastoolkit::CodeQL;
    ///
    /// let codeql = CodeQL::default();
    ///
    /// let languages = codeql.get_languages()
    ///     .expect("Failed to get languages");
    ///
    /// for language in languages {
    ///    println!("Language: {}", language.pretty());
    ///    // Do something with the language
    /// }
    ///
    pub fn get_languages(&self) -> Result<Vec<CodeQLLanguage>, GHASError> {
        Ok(self
            .get_all_languages()?
            .into_iter()
            .filter(|l| !l.is_secondary() || !l.is_none())
            .collect())
    }

    /// Get the secondary languages supported by the CodeQL CLI
    pub fn get_secondary_languages(&self) -> Result<Vec<CodeQLLanguage>, GHASError> {
        Ok(self
            .get_all_languages()?
            .into_iter()
            .filter(|l| l.is_secondary())
            .collect())
    }

    /// Get all languages supported by the CodeQL CLI
    pub fn get_all_languages(&self) -> Result<Vec<CodeQLLanguage>, GHASError> {
        match self.run(vec!["resolve", "languages", "--format", "json"]) {
            Ok(v) => {
                let languages: ResolvedLanguages = serde_json::from_str(&v)?;
                let mut result = Vec::new();
                for (language, _) in languages {
                    // allow custom languages if they come from CodeQL CLI
                    result.push(CodeQLLanguage::from((language.as_str(), true)));
                }
                result.sort();
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }
}

impl Display for CodeQL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.version() {
            Ok(v) => write!(f, "CodeQL('{}', '{}')", self.path.display(), v),
            Err(_) => write!(f, "CodeQL('{}')", self.path.display()),
        }
    }
}

impl Default for CodeQL {
    fn default() -> Self {
        CodeQL {
            path: CodeQL::find_codeql().unwrap_or_default(),
            threads: 0,
            ram: None,
            search_path: Vec::new(),
            additional_packs: Vec::new(),
        }
    }
}

/// CodeQL Builder to make it easier to create a new CodeQL instance
#[derive(Debug, Clone, Default)]
pub struct CodeQLBuilder {
    path: Option<String>,

    threads: usize,
    ram: usize,

    search_path: Vec<String>,
    additional_packs: Vec<String>,
}

impl CodeQLBuilder {
    /// Set the path to the CodeQL CLI
    pub fn path(mut self, path: String) -> Self {
        if !path.is_empty() {
            self.path = Some(path);
        }
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
    pub fn search_path(mut self, path: String) -> Self {
        self.search_path.push(path);
        self
    }

    /// Build the CodeQL instance
    pub fn build(&self) -> Result<CodeQL, GHASError> {
        let path: PathBuf = match self.path {
            Some(ref p) => PathBuf::from(p),
            None => match CodeQL::find_codeql() {
                Some(p) => p,
                None => PathBuf::new(),
            },
        };

        Ok(CodeQL {
            path,
            threads: self.threads,
            ram: self.ram.into(),
            additional_packs: self.additional_packs.clone(),
            search_path: self.search_path.clone(),
        })
    }
}
