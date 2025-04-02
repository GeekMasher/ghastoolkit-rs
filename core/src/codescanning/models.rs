use serde::{Deserialize, Serialize};

use crate::octokit::models::{Location, Message};

/// A code scanning alert.
/// https://docs.github.com/en/rest/code-scanning/code-scanning?apiVersion=2022-11-28#get-a-code-scanning-alert
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAlert {
    /// The ID of the alert.
    pub number: i32,
    /// Created at time.
    pub created_at: String,
    /// The URL of the alert.
    pub url: String,
    /// The HTML URL of the alert.
    pub html_url: String,
    /// The state of the alert. Can be "open", "fixed", etc.
    pub state: String,
    /// Fixed at time.
    pub fixed_at: Option<String>,
    /// Dismissed by user.
    pub dismissed_by: Option<CodeScanningAlertDismissedBy>,
    /// Dismissed at time.
    pub dismissed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Dismissed reason.
    pub dismissed_reason: Option<String>,
    /// Dismissed comment.
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

/// A code scanning alert rule.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAlertRule {
    /// The ID of the rule.
    pub id: String,
    /// The severity of the rule.
    pub severity: String,
    /// The tags of the rule.
    pub tags: Vec<String>,
    /// The description of the rule.
    pub description: String,
    /// The name of the rule.
    pub name: String,
}

/// A code scanning alert instance.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAlertInstance {
    /// The reference to the branch or tag the analysis was performed on.
    pub r#ref: String,
    /// Analysis key.
    pub analysis_key: String,
    /// Category.
    pub category: String,
    /// Environment.
    pub environment: String,
    /// The state of the alert instance.
    pub state: String,
    /// Commit SHA.
    pub commit_sha: String,
    /// Message.
    pub message: Message,
    /// Location.
    pub location: Location,
    /// Classifications.
    pub classifications: Vec<String>,
}

/// A code scanning alert tool.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAlertTool {
    /// The name of the tool.
    pub name: String,
    /// The guid of the tool.
    pub guid: Option<String>,
    /// The version of the tool.
    pub version: String,
}

/// A code scanning alert dismissed by.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAlertDismissedBy {}

/// A code scanning analysis.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeScanningAnalysis {
    /// The reference to the branch or tag the analysis was performed on.
    pub r#ref: String,
    /// The commit SHA the analysis was performed on.
    pub commit_sha: String,
    /// The analysis key.
    pub analysis_key: String,
    /// The environment the analysis was performed in.
    pub environment: String,
    /// Error message.
    pub error: Option<String>,
    /// Category.
    pub category: String,
    /// The time the analysis was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Number of alerts.
    pub results_count: i32,
    /// Rule count.
    pub rules_count: i32,
    /// ID of the analysis.
    pub id: i32,
    /// The URL of the analysis.
    pub url: String,
    /// SARIF ID.
    pub sarif_id: String,
    /// Code Scanning tool
    pub tool: CodeScanningAlertTool,
    /// Is the analysis deletable.
    pub deletable: bool,
    /// Warning message.
    pub warning: Option<String>,
}

/// A CodeQL Database
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct ListCodeQLDatabase {
    /// ID
    pub id: i32,
    /// Name
    pub name: String,
    /// Language
    pub language: String,
    /// Content Type
    pub content_type: String,
    /// Size
    pub size: i32,
    /// Created At
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Updated At
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
