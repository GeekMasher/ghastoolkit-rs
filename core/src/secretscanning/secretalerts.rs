//! # Secret Scanning Alerts
use std::fmt::Display;

use octocrab::models::SimpleUser;
use serde::{Deserialize, Serialize};
use url::Url;

/// Secret Scanning Alert Status
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum SecretScanningAlertStatus {
    /// Open Alert
    #[serde(rename = "open")]
    Open,
    /// Resolved Alert
    #[serde(rename = "resolved")]
    Resolved,
}

impl Display for SecretScanningAlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretScanningAlertStatus::Open => write!(f, "Open"),
            SecretScanningAlertStatus::Resolved => write!(f, "Resolved"),
        }
    }
}

/// Secret Scanning Alert Resolution
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum SecretScanningAlertResolution {
    /// Resolved as a False Positive
    #[serde(rename = "false_positive")]
    FalsePositive,
    /// Wont Fix
    #[serde(rename = "wont_fix")]
    WontFix,
    /// Revoked
    #[serde(rename = "revoked")]
    Revoked,
    /// Pattern Edited
    #[serde(rename = "pattern_edited")]
    PatternEdited,
    /// Pattern Deleted
    #[serde(rename = "pattern_deleted")]
    PatternDeleted,
    /// Used in Tests
    #[serde(rename = "used_in_tests")]
    UsedInTests,
}

/// Secret Scanning Validity
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum SecretScanningAlertValidity {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}

/// Secret Scanning Validity
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum SecretScanningSort {
    /// Active
    #[serde(rename = "created")]
    Created,
    /// Inactive
    #[serde(rename = "updated")]
    Updated,
}

/// A Secret Scanning Alert
///
/// https://docs.github.com/en/rest/secret-scanning/secret-scanning?apiVersion=2022-11-28
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct SecretScanningAlert {
    /// The ID of the alert
    pub number: u64,
    /// Creation time of the alert
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// State of the alert
    pub state: SecretScanningAlertStatus,

    /// Secret Scanning type
    pub secret_type: String,
    /// Secret Scanning type display name
    pub secret_type_display_name: String,

    /// Secret Value
    pub secret: String,

    /// Alert Resolution
    pub resolved: Option<SecretScanningAlertResolution>,
    /// When the alert was resolved
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Who resolved the alert
    pub resolved_by: Option<SimpleUser>,
    /// User resolution comment
    pub resolution_comment: Option<String>,

    /// Is Push Protection enabled
    pub push_protection_bypassed: Option<bool>,
    /// Who bypassed push protection
    pub push_protection_bypassed_by: Option<SimpleUser>,
    /// When it was bypassed
    pub push_protection_bypassed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Validity check
    pub validity: Option<SecretScanningAlertValidity>,

    /// URL
    pub url: Url,
    /// HTML
    pub html_url: Url,
    /// Locations
    pub locations_url: Url,
}
