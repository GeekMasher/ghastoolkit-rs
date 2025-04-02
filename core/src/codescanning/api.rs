use std::path::PathBuf;

use super::models::ListCodeQLDatabase;
use crate::{
    GHASError, Repository,
    codeql::CodeQLLanguage,
    codescanning::models::{CodeScanningAlert, CodeScanningAnalysis},
};
use log::debug;
use octocrab::{Octocrab, Page, Result as OctoResult};

/// Code Scanning Handler
#[derive(Debug, Clone)]
pub struct CodeScanningHandler<'octo> {
    crab: &'octo Octocrab,
    repository: &'octo Repository,
}

impl<'octo> CodeScanningHandler<'octo> {
    /// Create a new Code Scanning Handler instance
    pub(crate) fn new(crab: &'octo Octocrab, repository: &'octo Repository) -> Self {
        Self { crab, repository }
    }

    /// Check if GitHub Code Scanning is enabled. This is done by checking
    /// if the there is any analyses present for the repository.
    pub async fn is_enabled(&self) -> bool {
        match self.analyses().per_page(1).send().await {
            Ok(_) => true,
            Err(_) => {
                debug!("Code scanning is not enabled for this repository");
                false
            }
        }
    }

    /// Get a list of code scanning alerts for a repository
    pub fn list(&self) -> ListCodeScanningAlerts {
        ListCodeScanningAlerts::new(self)
    }

    /// Get a single code scanning alert
    pub async fn get(&self, number: u64) -> OctoResult<CodeScanningAlert> {
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/alerts/{number}",
            owner = self.repository.owner(),
            repo = self.repository.name(),
            number = number
        );

        self.crab.get(route, None::<&()>).await
    }

    /// Get a list of code scanning analyses for a repository
    pub fn analyses(&self) -> ListCodeScanningAnalyses {
        ListCodeScanningAnalyses::new(self)
    }

    /// List CodeQL databases
    pub async fn list_codeql_databases(&self) -> OctoResult<Vec<ListCodeQLDatabase>> {
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/codeql/databases",
            owner = self.repository.owner(),
            repo = self.repository.name()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Get a CodeQL database by language
    pub async fn get_codeql_database(&self, language: String) -> OctoResult<ListCodeQLDatabase> {
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/codeql/databases/{lang}",
            owner = self.repository.owner(),
            repo = self.repository.name(),
            lang = language
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Download a CodeQL database
    ///
    /// The Output is the root for where the database will be downloaded too.
    ///
    /// The output path will be something like this:
    /// ```
    /// output
    /// └── owner
    ///    └── repo
    ///       └── {language}
    /// ```
    ///
    /// Links:
    /// - https://docs.github.com/en/rest/code-scanning/code-scanning#get-a-codeql-database-for-a-repository
    ///
    pub async fn download_codeql_database(
        &self,
        language: impl Into<CodeQLLanguage>,
        output: impl Into<PathBuf>,
    ) -> Result<PathBuf, GHASError> {
        let language = language.into();
        let output = output.into();
        // Create the path
        let path = output
            .join(self.repository.owner())
            .join(self.repository.name())
            .join(language.language());
        let dbpath = path.join("codeql-database.zip");

        if path.exists() {
            // Remove the path as their might be an existing database
            std::fs::remove_dir_all(&path)?;
        }

        std::fs::create_dir_all(&path)?;
        log::info!("Downloading CodeQL database to {}", path.display());

        // TODO: Download the database
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/codeql/databases/{lang}",
            owner = self.repository.owner(),
            repo = self.repository.name(),
            lang = language.language()
        );
        let data = match self.crab.download_zip(route).await {
            Ok(data) => data,
            Err(err) => {
                log::error!("Failed to download CodeQL database");
                log::error!("{:?}", err);
                return Err(GHASError::CodeQLError(
                    "Failed to download CodeQL database".to_string(),
                ));
            }
        };

        tokio::fs::write(&dbpath, data).await?;

        self.unzip_codeql_database(&dbpath, &path)?;

        Ok(path)
    }

    /// Unzip the CodeQL database
    fn unzip_codeql_database(&self, zip: &PathBuf, output: &PathBuf) -> Result<(), GHASError> {
        log::debug!("Unzipping CodeQL database to {}", output.display());
        let file = std::fs::File::open(zip)?;
        let mut archive = zip::ZipArchive::new(file)?;
        archive.extract(output)?;

        Ok(())
    }
}

/// List Code Scanning Analyses
#[derive(Debug, serde::Serialize)]
pub struct ListCodeScanningAlerts<'octo, 'b> {
    #[serde(skip)]
    handler: &'b CodeScanningHandler<'octo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,
}

impl<'octo, 'b> ListCodeScanningAlerts<'octo, 'b> {
    pub(crate) fn new(handler: &'b CodeScanningHandler<'octo>) -> Self {
        Self {
            handler,
            state: Some(String::from("open")),
            tool_name: None,
            // Default to 100 per page
            per_page: Some(100),
            // Default to page 1
            page: Some(1),
        }
    }

    /// Set the state of the code scanning alert
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_string());
        self
    }

    /// Set the tool name of the code scanning alert
    pub fn tool_name(mut self, tool_name: &str) -> Self {
        self.tool_name = Some(tool_name.to_string());
        self
    }

    /// Set the number of items per page
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Set the page number
    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Send the request
    pub async fn send(self) -> OctoResult<Page<CodeScanningAlert>> {
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/alerts",
            owner = self.handler.repository.owner(),
            repo = self.handler.repository.name()
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}

/// List code scanning analyses
/// https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#list-code-scanning-analyses-for-a-repository
#[derive(Debug, serde::Serialize)]
pub struct ListCodeScanningAnalyses<'octo, 'b> {
    #[serde(skip)]
    handler: &'b CodeScanningHandler<'octo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r#ref: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sarif_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,
}

impl<'octo, 'b> ListCodeScanningAnalyses<'octo, 'b> {
    pub(crate) fn new(handler: &'b CodeScanningHandler<'octo>) -> Self {
        Self {
            handler,
            tool_name: None,
            r#ref: None,
            sarif_id: None,
            // Default to 100 per page
            per_page: Some(100),
            // Default to page 1
            page: Some(1),
        }
    }

    /// Set the ref of the code scanning analysis
    pub fn r#ref(mut self, r#ref: &str) -> Self {
        self.r#ref = Some(r#ref.to_string());
        self
    }

    /// Set the tool name of the code scanning analysis
    pub fn tool_name(mut self, tool_name: &str) -> Self {
        self.tool_name = Some(tool_name.to_string());
        self
    }

    /// Set the sarif id of the code scanning analysis
    pub fn sarif_id(mut self, sarif_id: &str) -> Self {
        self.sarif_id = Some(sarif_id.to_string());
        self
    }

    /// Set the number of items per page
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Set the page number
    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Send the request
    pub async fn send(self) -> OctoResult<Page<CodeScanningAnalysis>> {
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/analyses",
            owner = self.handler.repository.owner(),
            repo = self.handler.repository.name()
        );

        match self.handler.crab.get(route, Some(&self)).await {
            Ok(response) => Ok(response),
            Err(err) => Err(err),
        }
    }
}
