use serde::{Deserialize, Serialize};

use crate::octokit::models::{Location, Message};

/// A code scanning alert.
/// https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#get-a-code-scanning-alert
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAlert {
    pub number: i32,
    pub created_at: String,
    pub url: String,
    pub html_url: String,
    /// The state of the alert. Can be "open", "fixed", etc.
    pub state: String,
    pub fixed_at: Option<String>,
    pub dismissed_by: Option<CodeScanningAlertDismissedBy>,
    pub dismissed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub dismissed_reason: Option<String>,
    pub dismissed_comment: Option<String>,
    /// The rule that triggered the alert.
    pub rule: CodeScanningAlertRule,
    /// The tool that generated the alert.
    pub tool: CodeScanningAlertTool,
    /// The most recent instance of the alert.
    pub most_recent_instance: CodeScanningAlertInstance,
    /// URL to the instances of the alert.
    pub instances_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAlertRule {
    pub id: String,
    pub severity: String,
    pub tags: Vec<String>,
    pub description: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAlertInstance {
    pub r#ref: String,
    pub analysis_key: String,
    pub category: String,
    pub environment: String,
    pub state: String,
    pub commit_sha: String,
    pub message: Message,
    pub location: Location,
    pub classifications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAlertTool {
    pub name: String,
    pub guid: Option<String>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAlertDismissedBy {}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAnalysis {
    /// The reference to the branch or tag the analysis was performed on.
    pub r#ref: String,
    pub commit_sha: String,
    pub analysis_key: String,
    pub environment: String,
    pub error: Option<String>,
    pub category: String,
    /// The time the analysis was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub results_count: i32,
    pub rules_count: i32,
    pub id: i32,
    pub url: String,
    pub sarif_id: String,
    pub tool: CodeScanningAlertTool,
    pub deletable: bool,
    pub warning: Option<String>,
}
