//! # Code Scanning Configuration

use super::CodeScanningHandler;
use super::models::CodeScanningConfiguration;
use octocrab::Result as OctoResult;

impl<'octo> CodeScanningHandler<'octo> {
    /// Get the configuration for code scanning
    pub async fn get_configuration(&self) -> OctoResult<CodeScanningConfiguration> {
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/default-setup",
            owner = self.repository.owner(),
            repo = self.repository.name()
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Update the configuration for code scanning using a builder pattern
    ///
    /// *Example:*
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// use ghastoolkit::prelude::*;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// // Initialize a GitHub instance
    /// let github = GitHub::init()
    ///     .owner("geekmasher")
    ///     .token("personal_access_token")
    ///     .build()
    ///     .expect("Failed to initialise GitHub instance");
    ///
    /// // Create a repository instance
    /// let repository = Repository::new("geekmasher", "ghastoolkit-rs");
    /// // Use the builder to create a code scanning configuration
    /// github
    ///     .code_scanning(&repository)
    ///     .update_configuration()
    ///     .state("configured")
    ///     .language("rust")
    ///     .suite("default")
    ///     .threat_model("remote")
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn update_configuration(&self) -> CodeScanningConfigurationBuilder {
        CodeScanningConfigurationBuilder::new(self)
    }

    /// Set the configuration for code scanning
    ///
    /// *Example:*
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// use ghastoolkit::prelude::*;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// // Initialize a GitHub instance
    /// let github = GitHub::init()
    ///     .owner("geekmasher")
    ///     .token("personal_access_token")
    ///     .build()
    ///     .expect("Failed to initialise GitHub instance");
    ///
    /// // Create a repository instance
    /// let repository = Repository::new("geekmasher", "ghastoolkit-rs");
    ///
    /// // Create a code scanning configuration
    /// let config = CodeScanningConfiguration {
    ///     state: String::from("configured"),
    ///     languages: vec![String::from("rust")],
    ///     ..Default::default()
    /// };
    /// // Set the code scanning configuration
    /// github
    ///     .code_scanning(&repository)
    ///     .set_configuration(&config)
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_configuration(
        &self,
        config: &CodeScanningConfiguration,
    ) -> OctoResult<CodeScanningConfiguration> {
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/default-setup",
            owner = self.repository.owner(),
            repo = self.repository.name()
        );
        self.crab.patch(route, Some(&config)).await
    }
}

/// Code Scanning Configuration Builder
#[derive(Debug, Clone)]
pub struct CodeScanningConfigurationBuilder<'octo, 'handler> {
    handler: &'handler CodeScanningHandler<'octo>,
    config: CodeScanningConfiguration,
}

impl<'octo, 'handler> CodeScanningConfigurationBuilder<'octo, 'handler> {
    /// Create a new CodeScanningConfigurationBuilder
    pub fn new(handler: &'handler CodeScanningHandler<'octo>) -> Self {
        Self {
            handler,
            config: CodeScanningConfiguration::default(),
        }
    }

    /// Set the state of the code scanning configuration
    ///
    /// This can be "configured" or "non-configured".
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.config.state = state.into();
        self
    }

    /// Enable the code scanning configuration (equivalent to setting state to "configured")
    pub fn enable(mut self) -> Self {
        self.config.state = String::from("configured");
        self
    }

    /// Disable the code scanning configuration (equivalent to setting state to "non-configured")
    pub fn disable(mut self) -> Self {
        self.config.state = String::from("non-configured");
        self
    }

    /// Set a language for the code scanning configuration
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.config.languages.push(language.into());
        self
    }

    /// Set multiple languages for the code scanning configuration
    pub fn languages(mut self, languages: Vec<String>) -> Self {
        self.config.languages = languages;
        self
    }

    /// Set the query suite for the code scanning configuration
    ///
    /// This can be either "default" or "extended".
    pub fn suite(mut self, suite: impl Into<String>) -> Self {
        self.config.query_suite = suite.into();
        self
    }

    /// Set the threat model for the code scanning configuration
    ///
    /// This can be either "remote" or "remote_and_local".
    pub fn threat_model(mut self, threat_model: impl Into<String>) -> Self {
        self.config.threat_model = threat_model.into();
        self
    }

    /// Set the thread model to "remote"
    pub fn remote(mut self) -> Self {
        self.config.threat_model = String::from("remote");
        self
    }

    /// Set the thread model to "remote_and_local"
    pub fn remote_and_local(mut self) -> Self {
        self.config.threat_model = String::from("remote_and_local");
        self
    }

    /// Set the GitHUb Action runner type for the code scanning configuration
    ///
    /// Can either be "standard" or "labeled"
    pub fn runner_type(mut self, runner_type: impl Into<String>) -> Self {
        self.config.runner_type = Some(runner_type.into());
        self
    }

    /// Set the GitHub Action runner label for the code scanning configuration
    pub fn runner_label(mut self, runner_label: impl Into<String>) -> Self {
        self.config.runner_label = Some(runner_label.into());
        self.config.runner_type = Some(String::from("labeled"));
        self
    }

    /// Build the code scanning configuration
    pub fn build(self) -> CodeScanningConfiguration {
        self.config
    }

    /// Send the request
    pub async fn send(self) -> OctoResult<()> {
        self.handler.set_configuration(&self.config).await?;
        Ok(())
    }
}
