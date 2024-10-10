//! # Secret Scanning Alert

use octocrab::{Octocrab, Page, Result as OctoResult};

use crate::Repository;

use super::secretalerts::{SecretScanningAlert, SecretScanningSort};

/// Secret Scanning Handler
#[derive(Debug, Clone)]
pub struct SecretScanningHandler<'octo> {
    crab: &'octo Octocrab,
    repository: &'octo Repository,
}

impl<'octo> SecretScanningHandler<'octo> {
    /// Create a new Code Scanning Handler instance
    pub(crate) fn new(crab: &'octo Octocrab, repository: &'octo Repository) -> Self {
        Self { crab, repository }
    }

    /// Get a list of code scanning alerts for a repository
    pub fn list(&self) -> ListSecretScanningAlerts {
        ListSecretScanningAlerts::new(self)
    }

    /// Get a single code scanning alert
    pub async fn get(&self, number: u64) -> OctoResult<SecretScanningAlert> {
        let route = format!(
            "/repos/{owner}/{repo}/secret-scanning/alerts/{number}",
            owner = self.repository.owner(),
            repo = self.repository.name(),
            number = number
        );

        self.crab.get(route, None::<&()>).await
    }
}

/// List Secret Scanning Alerts
#[derive(Debug, serde::Serialize)]
pub struct ListSecretScanningAlerts<'octo, 'b> {
    #[serde(skip)]
    handler: &'b SecretScanningHandler<'octo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    secret_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SecretScanningSort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    validity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,
}

impl<'octo, 'b> ListSecretScanningAlerts<'octo, 'b> {
    pub(crate) fn new(handler: &'b SecretScanningHandler<'octo>) -> Self {
        Self {
            handler,
            state: Some(String::from("open")),
            secret_type: None,
            sort: None,
            validity: None,
            // Default to 100 per page
            per_page: Some(25),
            // Default to page 1
            page: Some(1),
        }
    }

    /// Set the state of the code scanning alert
    pub fn state(mut self, state: impl Into<String>) -> Self {
        let state = state.into();
        if !state.is_empty() {
            self.state = Some(state);
        }
        self
    }

    /// Set the Secret Type
    pub fn secret_type(mut self, stype: impl Into<String>) -> Self {
        self.secret_type = Some(stype.into());
        self
    }

    /// Sort
    pub fn sort(mut self, sort: impl Into<SecretScanningSort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Validity
    pub fn validity(mut self, validity: impl Into<String>) -> Self {
        self.validity = Some(validity.into());
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
    pub async fn send(self) -> OctoResult<Page<SecretScanningAlert>> {
        let route = format!(
            "/repos/{owner}/{repo}/secret-scanning/alerts",
            owner = self.handler.repository.owner(),
            repo = self.handler.repository.name()
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}
