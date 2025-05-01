use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::GHASError;

/// Sarif Structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sarif {
    /// Schema URL
    #[serde(rename = "$schema")]
    pub schema: String,
    /// Schema Version
    pub version: String,
    /// Runs
    pub runs: Vec<SarifRun>,
}

impl TryFrom<PathBuf> for Sarif {
    type Error = GHASError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        // Read and load SARIF file
        let file = std::fs::File::open(value)?;
        let reader = std::io::BufReader::new(file);
        let sarif: Sarif = serde_json::from_reader(reader)?;
        Ok(sarif)
    }
}

impl TryFrom<String> for Sarif {
    type Error = GHASError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Sarif::try_from(PathBuf::from(value))
    }
}

impl Sarif {
    /// Create a new SARIF object
    pub fn new() -> Self {
        Sarif {
            schema: String::from(
                "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            ),
            version: String::from("2.1.0"),
            runs: vec![],
        }
    }

    /// Get Results from all runs
    pub fn get_results(&self) -> Vec<SarifResult> {
        let mut results = vec![];
        for run in &self.runs {
            results.extend(run.results.clone());
        }
        results
    }

    /// Write SARIF to file
    pub fn write(&self, path: PathBuf) -> Result<(), GHASError> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }
}

/// Sarif Run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRun {
    /// Tool
    pub tool: SarifTool,
    /// Results
    pub results: Vec<SarifResult>,
}

/// Sarif Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifResult {
    /// Rule ID
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    /// Rule Index
    #[serde(rename = "ruleIndex")]
    pub rule_index: i32,
    /// Rule
    pub rule: SarifRule,
    /// Level
    pub level: String,
    /// Message
    pub message: SarifMessage,
    /// Locations
    pub locations: Vec<SarifLocation>,
}

impl Display for SarifResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.rule_id, self.message.text)
    }
}

/// SARIF Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRule {
    /// ID
    pub id: String,
    /// Index
    pub index: i32,
}

/// SARIF Location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifLocation {
    /// Physical Location
    #[serde(rename = "physicalLocation")]
    pub physical_location: SarifPhysicalLocation,
}

/// SARIF Physical Location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifPhysicalLocation {
    /// Artifact Location
    #[serde(rename = "artifactLocation")]
    pub artifact_location: SarifArtifactLocation,
    /// Region
    pub region: SarifRegion,
}

/// SARIF Artifact Location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifArtifactLocation {
    /// URI
    pub uri: String,
    /// URI Base ID
    #[serde(rename = "uriBaseId")]
    pub uri_base_id: String,
    /// ID
    pub id: i32,
}

/// SARIF Region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRegion {
    /// Start Line
    #[serde(rename = "startLine")]
    pub start_line: i32,
    /// Start Column
    #[serde(rename = "startColumn")]
    pub start_column: i32,
    /// End Line
    #[serde(rename = "endLine")]
    pub end_line: Option<i32>,
    /// End Column
    #[serde(rename = "endColumn")]
    pub end_column: Option<i32>,
}

/// SARIF Tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifTool {
    /// Driver
    pub driver: SarifToolDriver,
}

impl Display for SarifTool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(version) = &self.driver.version {
            write!(f, "{} v{}", self.driver.name, version)
        } else {
            write!(f, "{}", self.driver.name)
        }
    }
}

/// SARIF Tool Driver
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifToolDriver {
    /// Name
    pub name: String,
    /// Organization
    pub organization: Option<String>,
    /// Version
    #[serde(rename = "semanticVersion")]
    pub version: Option<String>,
    /// Notifications
    pub notifications: Option<Vec<SarifToolDriverNotification>>,
}

/// SARIF Tool Driver Notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifToolDriverNotification {
    /// Identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Short Description
    #[serde(rename = "shortDescription")]
    pub short_description: SarifMessage,
    /// Full Description
    #[serde(rename = "fullDescription")]
    pub full_description: SarifMessage,
}

/// SARIF Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifMessage {
    /// Text
    pub text: String,
}
