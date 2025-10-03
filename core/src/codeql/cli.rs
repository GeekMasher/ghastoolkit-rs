//! # CodeQL CLI Wrapper

use log::debug;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::{
    CodeQLDatabase, CodeQLPack, GHASError,
    codeql::{CodeQLLanguage, database::handler::CodeQLDatabaseHandler},
    utils::sarif::Sarif,
};

pub mod builder;
mod models;
pub mod packs;

use super::{CodeQLExtractor, languages::CodeQLLanguages, packs::handler::CodeQLPackHandler};
pub use builder::CodeQLBuilder;
use models::ResolvedLanguages;

/// CodeQL CLI Wrapper to make it easier to run CodeQL commands
#[derive(Debug, Clone)]
pub struct CodeQL {
    /// CodeQL CLI Version
    version: Option<String>,
    /// Path to the CodeQL CLI
    path: PathBuf,
    /// Number of threads to use
    threads: usize,
    /// Amount of RAM to use
    ram: Option<usize>,
    /// The search path for the CodeQL CLI
    search_path: Vec<PathBuf>,
    /// Additional packs to use
    additional_packs: Vec<String>,

    /// Token to use for authentication with CodeQL registries
    token: Option<String>,

    /// Default Suite to use if not specified
    pub(crate) suite: Option<String>,

    /// Shows the output of the command
    showoutput: bool,
}

impl CodeQL {
    /// Create a new CodeQL instance
    #[cfg(not(feature = "async"))]
    pub fn new() -> Self {
        CodeQL::default()
    }

    /// Create a new CodeQL instance
    #[cfg(feature = "async")]
    pub async fn new() -> Self {
        let path = CodeQL::find_codeql().await.unwrap_or_default();

        CodeQL {
            version: CodeQL::get_version(&path).await.ok(),
            path,
            threads: 0,
            ram: None,
            search_path: Vec::new(),
            additional_packs: Vec::new(),
            token: None,
            suite: None,
            showoutput: true,
        }
    }

    /// Get the CodeQL CLI path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Set the CodeQL CLI path
    pub(crate) fn set_path(&mut self, path: PathBuf) {
        log::trace!("Setting CodeQL path to {:?}", path);
        self.path = path;
    }

    /// Initialize a new CodeQL Builder instance
    pub fn init() -> CodeQLBuilder {
        CodeQLBuilder::default()
    }

    /// Get the search paths set for the CodeQL CLI to use.
    ///
    /// Paths are separated by a colon
    pub(crate) fn search_paths(&self) -> String {
        self.search_path
            .iter()
            .map(|p| p.to_str().unwrap().to_string())
            .collect::<Vec<String>>()
            .join(":")
    }

    /// Add the search path to the CodeQL CLI arguments
    pub(crate) fn add_search_path(&self, args: &mut Vec<String>) {
        if !self.search_path.is_empty() {
            args.push("--search-path".to_string());
            args.push(self.search_paths());
        }
    }

    /// Append a search path to the CodeQL CLI
    pub fn append_search_path(&mut self, path: impl Into<PathBuf>) {
        self.search_path.push(path.into());
    }

    /// Add the additional packs to the CodeQL CLI arguments
    pub(crate) fn add_additional_packs(&self, args: &mut Vec<String>) {
        if !self.additional_packs.is_empty() {
            args.push("--additional-packs".to_string());
            args.push(self.additional_packs.join(","));
        }
    }

    /// Get the default suite for the CodeQL CLI
    pub fn default_suite(&self) -> String {
        self.suite.clone().unwrap_or("code-scanning".to_string())
    }
    /// Add an external extractor to the CodeQL CLI (search path)
    pub fn add_extractor(&mut self, extractor: &CodeQLExtractor) {
        self.search_path.push(extractor.path.clone());
    }

    /// Find CodeQL CLI on the system (asynchronous)
    pub async fn find_codeql() -> Option<PathBuf> {
        // Root CodeQL Paths
        if let Some(e) = std::env::var_os("CODEQL_PATH") {
            let p = PathBuf::from(e).join("codeql");
            if p.exists() && p.is_file() {
                return Some(p);
            }
        } else if let Some(e) = std::env::var_os("CODEQL_BINARY") {
            let p = PathBuf::from(e);
            if p.exists() && p.is_file() {
                return Some(p);
            }
        }
        #[cfg(feature = "toolcache")]
        {
            if let Some(t) = CodeQL::find_codeql_toolcache().await {
                log::debug!("Found CodeQL in toolcache: {:?}", t);
                return Some(t);
            }
        }
        if let Some(p) = CodeQL::find_codeql_path() {
            log::debug!("Found CodeQL in PATH: {:?}", p);
            return Some(p);
        }

        None
    }

    /// Load a CodeQL extractor from a path
    pub async fn load_extractor(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> Result<CodeQLExtractor, GHASError> {
        let path = path.into();

        let extractor = CodeQLExtractor::load_path(&path)?;
        self.search_path.push(path);

        Ok(extractor)
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

    #[cfg(feature = "toolcache")]
    async fn find_codeql_toolcache() -> Option<PathBuf> {
        let toolcache = ghactions::ToolCache::new();
        if let Ok(tool) = toolcache.find("CodeQL", "latest").await {
            let tool = tool.path();
            // TODO: This needs to be better
            if tool.join("codeql").is_file() {
                return Some(tool.join("codeql"));
            } else if tool.join("codeql").is_dir() && tool.join("codeql").join("codeql").is_file() {
                return Some(tool.join("codeql").join("codeql"));
            }
        }
        None
    }

    /// Run a CodeQL command asynchronously
    /// 
    /// This function will run the CodeQL command with the given arguments and return the output.
    /// 
    /// It will also set the `CODEQL_REGISTRIES_AUTH` environment variable if a token is provided.
    pub async fn run(&self, args: Vec<impl AsRef<str>>) -> Result<String, GHASError> {
        let args: Vec<String> = args.iter().map(|arg| arg.as_ref().to_string()).collect();
        debug!("CodeQL::run({:?})", args);

        // Insert CODEQL_REGISTRIES_AUTH to the env
        let mut envs = std::env::vars_os()
            .map(|(k, v)| (k.to_string_lossy().to_string(), v))
            .collect::<std::collections::HashMap<String, std::ffi::OsString>>();
        if let Some(token) = &self.token {
            envs.insert(
                "CODEQL_REGISTRIES_AUTH".to_string(),
                token.to_string().into()
            );
        }


        let mut cmd = tokio::process::Command::new(&self.path)
            .args(args)
            .envs(envs)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;

        let stdout = cmd.stdout.take().ok_or(GHASError::CodeQLError(
            "Failed to get stdout from CodeQL command".to_string(),
        ))?;

        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        let mut output_lines = Vec::new();

        while let Some(line) = lines.next_line().await? {
            if self.showoutput {
                println!("{}", line); // print live
            }
            output_lines.push(line); // store for later
        }

        let status = cmd.wait().await?;

        if status.success() {
            debug!("CodeQL Command Success: {:?}", status.to_string());
            Ok(output_lines.join("\n"))
        } else {
            Err(GHASError::CodeQLError(
                "Error running CodeQL command".to_string(),
            ))
        }
    }

    /// Run a CodeQL command without showing the output
    ///
    /// This is an internal function and should not be used directly.
    pub(crate) async fn rn(&self, args: Vec<impl AsRef<str>>) -> Result<String, GHASError> {
        let mut codeql = self.clone();
        codeql.showoutput = false;
        codeql.run(args).await
    }

    /// Pass a CodeQLDatabase to the CodeQL CLI to return a CodeQLDatabaseHandler.
    /// This handler can be used to run queries and other operations on the database.
    #[allow(elided_named_lifetimes)]
    pub fn database<'a>(&'a self, db: &'a CodeQLDatabase) -> CodeQLDatabaseHandler {
        CodeQLDatabaseHandler::new(db, self)
    }

    /// Pass a CodeQLPack to the CodeQL CLI to return a CodeQLPackHandler.
    ///
    /// This handler can be used to run queries and other operations on the pack.
    #[allow(elided_named_lifetimes)]
    pub fn pack<'a>(&'a self, pack: &'a CodeQLPack) -> CodeQLPackHandler {
        CodeQLPackHandler::new(pack, self)
    }

    /// An async function to run a CodeQL scan on a database.
    ///
    /// This includes the following steps:
    /// - Creating the database
    /// - Running the analysis
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ghastoolkit::codeql::{CodeQL, CodeQLDatabase};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let codeql = CodeQL::new().await;
    ///
    /// let mut db = CodeQLDatabase::init()
    ///     .source("./")
    ///     .language("python")
    ///     .build()
    ///     .expect("Failed to create database");
    ///
    /// let sarif = codeql.scan(&mut db, "codeql/python-queries").await
    ///     .expect("Failed to run scan");
    /// // ... do something with the sarif
    /// # }
    /// ```
    pub async fn scan<'a>(
        &'a self,
        db: &'a mut CodeQLDatabase,
        queries: impl Into<String>,
    ) -> Result<Sarif, GHASError> {
        self.database(db).overwrite().create().await?;

        let sarif = db.path().join("results.sarif");
        self.database(db)
            .sarif(sarif.clone())
            .queries(queries.into())
            .analyze()
            .await?;

        db.reload()?;

        self.sarif(sarif)
    }

    /// Get the SARIF file from the CodeQL CLI
    pub fn sarif(&self, path: impl Into<PathBuf>) -> Result<Sarif, GHASError> {
        Ok(Sarif::try_from(path.into())?)
    }

    /// Get the version of the loaded CodeQL CLI
    pub fn version(&self) -> Option<String> {
        self.version.clone()
    }

    /// Check to see if the CodeQL CLI is installed
    pub async fn is_installed(&self) -> bool {
        Self::get_version(&self.path).await.is_ok()
    }

    /// Get the version of the CodeQL CLI
    pub async fn get_version(path: &Path) -> Result<String, GHASError> {
        log::debug!("CodeQL.get_version path :: {:?}", path);
        let output = tokio::process::Command::new(path)
            .args(["version", "--format", "terse"])
            .output()
            .await?;

        if output.status.success() {
            debug!("CodeQL Command Success: {:?}", output.status.to_string());
            Ok(String::from_utf8_lossy(&output.stdout)
                .to_string()
                .trim()
                .to_string())
        } else {
            Err(GHASError::CodeQLError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
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
    /// # #[tokio::main]
    /// # async fn main() {
    /// let codeql = CodeQL::default();
    ///
    /// let languages = codeql.get_languages()
    ///     .await
    ///     .expect("Failed to get languages");
    ///
    /// for language in languages {
    ///    println!("Language: {}", language.pretty());
    ///    // Do something with the language
    /// }
    ///
    /// # }
    /// ```
    pub async fn get_languages(&self) -> Result<Vec<CodeQLLanguage>, GHASError> {
        Ok(self.get_all_languages().await?.get_languages())
    }

    /// Get the secondary languages supported by the CodeQL CLI
    pub async fn get_secondary_languages(&self) -> Result<Vec<CodeQLLanguage>, GHASError> {
        Ok(self.get_all_languages().await?.get_secondary())
    }

    /// Get all languages supported by the CodeQL CLI
    pub async fn get_all_languages(&self) -> Result<CodeQLLanguages, GHASError> {
        let mut args = vec!["resolve", "languages", "--format", "json"];

        let search_path = self.search_paths();
        if !self.search_path.is_empty() {
            args.push("--search-path");
            args.push(&search_path);
        }

        log::debug!("CodeQL.get_all_languages args :: {:?}", args);

        match self.rn(args).await {
            Ok(v) => {
                let languages: ResolvedLanguages = serde_json::from_str(&v)?;
                let mut result = Vec::new();
                for (language, path) in languages {
                    // allow custom languages if they come from CodeQL CLI
                    result.push(CodeQLLanguage::from((
                        language,
                        PathBuf::from(path.first().unwrap()),
                    )));
                }
                result.sort();
                Ok(CodeQLLanguages::new(result))
            }
            Err(e) => Err(e),
        }
    }
}

impl Display for CodeQL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(version) = &self.version {
            write!(f, "CodeQL('{}', '{}')", self.path.display(), version)
        } else {
            write!(f, "CodeQL('{}')", self.path.display())
        }
    }
}

impl Default for CodeQL {
    fn default() -> Self {
        CodeQL {
            version: None,
            path: PathBuf::new(),
            threads: 0,
            ram: None,
            search_path: Vec::new(),
            additional_packs: Vec::new(),
            token: None,
            suite: None,
            showoutput: true,
        }
    }
}
