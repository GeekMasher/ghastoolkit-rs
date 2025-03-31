use std::{fmt::Display, path::PathBuf};

use git2::Repository as GitRepository;
use log::debug;
use octocrab::{Octocrab, Result as OctoResult};
use url::Url;

use crate::{
    GHASError, Repository, codescanning::api::CodeScanningHandler,
    octokit::models::GitHubLanguages, secretscanning::api::SecretScanningHandler,
};

/// GitHub instance
#[derive(Debug, Clone)]
pub struct GitHub {
    /// Octocrab instance
    octocrab: Octocrab,

    /// Owner of the repository (organization or user)
    owner: Option<String>,
    /// Enterprise account name (if applicable)
    enterprise: Option<String>,

    /// GitHub token (personal access token or GitHub App token)
    token: Option<String>,

    /// GitHub instance (e.g. https://github.com or enterprise server instance)
    instance: Url,
    /// REST API endpoint
    api_rest: Url,

    /// If an enterprise server instance is being used
    enterprise_server: bool,

    /// If the token is for a GitHub App
    github_app: bool,
}

impl GitHub {
    /// Initialize a new GitHub instance with default values
    pub fn new() -> Self {
        GitHub::default()
    }

    /// Initialize a new GitHub instance with a builder pattern
    ///
    /// # Example
    /// ```rust
    /// use ghastoolkit::GitHub;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let github = GitHub::init()
    ///     .owner("geekmasher")
    ///     .token("personal_access_token")
    ///     .build()
    ///     .expect("Failed to initialise GitHub instance");
    /// # }
    /// ```
    pub fn init() -> GitHubBuilder {
        GitHubBuilder::default()
    }

    /// Check if the GitHub instance is for Enterprise Server or Cloud.
    /// This is done based off the URL provided.
    pub fn is_enterprise_server(&self) -> bool {
        self.enterprise_server
    }

    /// Get the GitHub instance URL as a String
    pub fn instance(&self) -> String {
        self.instance.to_string()
    }

    /// Get the GitHub Token
    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }

    /// Get the URL used for clong a repository.
    fn clone_repository_url(&self, repo: &Repository) -> Result<String, GHASError> {
        if self.github_app {
            // GitHub Apps require a different URL
            Ok(format!(
                "{}://x-access-token:{}@{}/{}/{}.git",
                self.instance.scheme(),
                self.token.clone().expect("Failed to get token"),
                self.instance.host().expect("Failed to get host"),
                repo.owner(),
                repo.name()
            ))
        } else if let Some(token) = &self.token {
            Ok(format!(
                "{}://{}@{}/{}/{}.git",
                self.instance.scheme(),
                token,
                self.instance.host().expect("Failed to get host"),
                repo.owner(),
                repo.name()
            ))
        } else {
            // No token
            Ok(format!(
                "{}://{}/{}/{}.git",
                self.instance.scheme(),
                self.instance.host().expect("Failed to get host"),
                repo.owner(),
                repo.name()
            ))
        }
    }

    /// Get the pre-build instance of Octocrab.
    /// This has automatically configured the base URI, PAT, and other settings.
    ///
    /// # Example
    /// ```no_run
    /// # use anyhow::Result;
    /// use ghastoolkit::GitHub;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let github = GitHub::init()
    ///     .owner("geekmasher")
    ///     .token("personal_access_token")
    ///     .build()
    ///     .expect("Failed to initialise GitHub instance");
    ///
    /// let octocrab = github.octocrab();
    ///
    /// let issues = octocrab.issues("geekmasher", "ghastoolkit-rs")
    ///     .list()
    ///     .state(octocrab::params::State::Open)
    ///     .send()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn octocrab(&self) -> &octocrab::Octocrab {
        &self.octocrab
    }

    /// Get Secret Scanning Handler based on the Repository
    #[allow(elided_named_lifetimes)]
    pub fn secret_scanning<'a>(&'a self, repo: &'a Repository) -> SecretScanningHandler {
        SecretScanningHandler::new(self.octocrab(), repo)
    }

    /// Get Code Scanning Handler based on the Repository provided.
    #[allow(elided_named_lifetimes)]
    pub fn code_scanning<'a>(&'a self, repo: &'a Repository) -> CodeScanningHandler {
        CodeScanningHandler::new(self.octocrab(), repo)
    }

    /// Get Repository languages from GitHub
    pub async fn list_languages(&self, repo: &Repository) -> OctoResult<GitHubLanguages> {
        let route = format!("/repos/{}/{}/languages", repo.owner(), repo.name());
        self.octocrab.get(route, None::<&()>).await
    }

    /// Clone a GitHub Repository to a local path
    pub fn clone_repository(
        &self,
        repo: &mut Repository,
        path: &String,
    ) -> Result<GitRepository, GHASError> {
        let url = self.clone_repository_url(repo)?;
        match GitRepository::clone(url.as_str(), path.as_str()) {
            Ok(gitrepo) => {
                repo.set_root(PathBuf::from(path));
                Ok(gitrepo)
            }
            Err(e) => Err(GHASError::from(e)),
        }
    }
}

impl Display for GitHub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GitHub(instance: {:?}, owner: '{:?}', enterprise: {:?})",
            self.instance.to_string(),
            self.owner,
            self.enterprise,
        )
    }
}

impl Default for GitHub {
    /// GitHub defaults to using Environment Variables for the GitHub instance and token.
    fn default() -> Self {
        let instance = match std::env::var("GITHUB_INSTANCE") {
            Ok(val) => Url::parse(val.as_str()).expect("Failed to parse GitHub instance URL"),
            Err(_) => {
                Url::parse("https://github.com").expect("Failed to parse GitHub instance URL")
            }
        };
        // TODO(geekmasher): REST API
        let token = match std::env::var("GITHUB_TOKEN") {
            Ok(val) => Some(val),
            Err(_) => None,
        };

        Self {
            octocrab: Octocrab::default(),
            owner: None,
            enterprise: None,
            token,
            instance,
            api_rest: Url::parse("https://api.github.com")
                .expect("Failed to parse GitHub REST API URL"),
            enterprise_server: false,
            github_app: false,
        }
    }
}

/// GitHub Builder
#[derive(Debug, Clone)]
pub struct GitHubBuilder {
    owner: Option<String>,
    enterprise: Option<String>,
    token: Option<String>,
    instance: Url,
    rest_api: Url,
    enterprise_server: bool,
    github_app: bool,
}

impl GitHubBuilder {
    /// Set the Instance URL for the GitHub Enterprise Server.
    ///
    /// # Example
    /// ```rust
    /// use ghastoolkit::GitHub;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let github = GitHub::init()
    ///     .instance("https://github.geekmasher.dev/")
    ///     .build()
    ///     .expect("Failed to initialise GitHub instance");
    ///
    /// # assert_eq!(github.instance(), "https://github.geekmasher.dev/");
    /// # }
    /// ```
    pub fn instance(&mut self, instance: &str) -> &mut Self {
        self.instance = Url::parse(instance).expect("Failed to parse instance URL");

        // GitHub Cloud
        if self.instance.host_str() == Some("github.com") {
            self.rest_api =
                Url::parse("https://api.github.com").expect("Failed to parse REST API URL");
            self.enterprise_server = false;
        } else {
            // GitHub Enterprise Server endpoint
            self.rest_api = Url::parse(format!("{}/api/v3", instance).as_str())
                .expect("Failed to parse REST API URL");
            self.enterprise_server = true;
        }

        self
    }
    /// Set the Token used to authenticate with GitHub.
    ///
    /// # Example
    /// ```rust
    /// use ghastoolkit::GitHub;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let github = GitHub::init()
    ///     .token("personal_access_token")
    ///     .build()
    ///     .expect("Failed to initialise GitHub instance");
    ///
    /// # assert_eq!(github.token(), Some(&String::from("personal_access_token")));
    /// # }
    /// ```
    pub fn token(&mut self, token: &str) -> &mut Self {
        self.token = Some(token.to_string());
        self
    }

    /// Set the Owner (Username or Organization).
    pub fn owner(&mut self, owner: &str) -> &mut Self {
        if !owner.is_empty() {
            self.owner = Some(owner.to_string());
        }
        self
    }

    /// Set the Enterprise Account name.
    pub fn enterprise(&mut self, enterprise: &str) -> &mut Self {
        self.enterprise = Some(enterprise.to_string());
        self
    }

    /// Set the GitHub App flag. This is mainly used for changing the rate limits and other
    /// settings.
    pub fn github_app(&mut self, github_app: bool) -> &mut Self {
        self.github_app = github_app;
        self
    }

    /// Build the GitHub instance with the provided settings.
    ///
    /// # Example
    /// ```rust
    /// use ghastoolkit::GitHub;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let github = GitHub::init()
    ///     .instance("https://github.geekmasher.dev/")
    ///     .owner("geekmasher")
    ///     .token("personal_access_token")
    ///     .build()
    ///     .expect("Failed to initialise GitHub instance");
    /// # }
    /// ```
    pub fn build(&self) -> Result<GitHub, GHASError> {
        let token = match self.token.clone() {
            Some(token) => Some(token),
            None => std::env::var("GITHUB_TOKEN").ok(),
        };

        let mut builder = octocrab::Octocrab::builder();

        if let Some(token) = &self.token {
            debug!("Setting personal token");
            builder = builder.personal_token(token.clone());
        }

        debug!("Setting base URI to: {}", self.rest_api);
        builder = builder
            .base_uri(self.rest_api.to_string().as_str())
            .expect("Failed to set base URI");

        Ok(GitHub {
            octocrab: builder.build().expect("Failed to build Octocrab instance"),
            owner: self.owner.clone(),
            enterprise: self.enterprise.clone(),
            token,
            instance: self.instance.clone(),
            api_rest: self.rest_api.clone(),
            enterprise_server: self.enterprise_server,
            github_app: self.github_app,
        })
    }
}

impl Default for GitHubBuilder {
    fn default() -> Self {
        Self {
            owner: None,
            enterprise: None,
            token: None,
            instance: Url::parse("https://github.com")
                .expect("Failed to parse GitHub instance URL"),
            rest_api: Url::parse("https://api.github.com")
                .expect("Failed to parse GitHub REST API URL"),
            enterprise_server: false,
            github_app: false,
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_github_builder() {
        let gh = GitHub::init()
            .instance("https://github.com")
            .token("token")
            .owner("geekmasher")
            .build()
            .expect("Failed to build GitHub instance");

        assert_eq!(gh.instance, Url::parse("https://github.com").unwrap());
        assert_eq!(gh.token, Some("token".to_string()));
        assert_eq!(gh.owner, Some("geekmasher".to_string()));
    }

    #[tokio::test]
    async fn test_repo_clone_url() {
        let gh = GitHub::init()
            .instance("https://github.com")
            .token("token")
            .owner("geekmasher")
            .build()
            .expect("Failed to build GitHub instance");
        let repo = Repository::try_from("geekmasher/ghastoolkit@main")
            .expect("Failed to parse repository");

        let url = gh
            .clone_repository_url(&repo)
            .expect("Failed to get clone URL");
        assert_eq!(url, "https://token@github.com/geekmasher/ghastoolkit.git");
    }
}
