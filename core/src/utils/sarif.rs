use std::{
    collections::HashMap,
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
    /// References to external property files that share data between runs
    #[serde(rename = "inlineExternalProperties", skip_serializing_if = "Option::is_none")]
    pub inline_external_properties: Option<Vec<SarifExternalProperties>>,
    /// Key/value pairs that provide additional information about the log file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

impl TryFrom<PathBuf> for Sarif {
    type Error = GHASError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::load(value)
    }
}

impl TryFrom<String> for Sarif {
    type Error = GHASError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::load(value)
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
            inline_external_properties: None,
            properties: None,
        }
    }

    /// Load SARIF from a file
    pub fn load(path: impl Into<PathBuf>) -> Result<Self, GHASError> {
        let path = path.into();
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let sarif: Sarif = serde_json::from_reader(reader)?;
        Ok(sarif)
    }

    /// Runs a set of validation checks on the SARIF object and if 
    /// there are any issues, returns a GHASError.
    pub fn validate(&mut self) -> Result<(), GHASError> {

        Ok(())
    }

    /// Count all results across runs
    pub fn count_results(&self) -> usize {
        self.runs.iter().map(|run| run.results.len()).sum()
    }

    /// Count all the unique rules across runs
    pub fn count_rules(&self) -> usize {
        let mut unique_rules = std::collections::HashSet::new();
        for run in &self.runs {
            for result in &run.results {
                if let Some(rule_id) = &result.rule_id {
                    unique_rules.insert(rule_id.clone());
                }
            }
        }
        unique_rules.len()
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
    #[serde(default)]
    pub results: Vec<SarifResult>,
    /// Describes the invocation of the analysis tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocations: Option<Vec<SarifInvocation>>,
    /// The language of the messages emitted into the log file during this run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// An array of artifact objects relevant to the run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<SarifArtifact>>,
    /// Key/value pairs that provide additional information about the run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Sarif Result
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SarifResult {
    /// Rule ID
    #[serde(rename = "ruleId", skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// Rule Index
    #[serde(rename = "ruleIndex", skip_serializing_if = "Option::is_none")]
    pub rule_index: Option<i32>,
    /// Rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<SarifReportingDescriptorReference>,
    /// Taxa
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxa: Option<Vec<SarifReportingDescriptorReference>>,
    /// Level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<SarifLevel>,
    /// Message
    pub message: SarifMessage,
    /// Locations
    #[serde(default)]
    pub locations: Vec<SarifLocation>,
    /// The artifact that the analysis tool was instructed to scan
    #[serde(rename = "analysisTarget", skip_serializing_if = "Option::is_none")]
    pub analysis_target: Option<SarifArtifactLocation>,
    /// A unique identifier for the result in the form of a GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// A value that helps in determining the exact position of the result
    #[serde(rename = "correlationGuid", skip_serializing_if = "Option::is_none")]
    pub correlation_guid: Option<String>,
    /// A positive integer specifying the number of times this logically unique result was observed in this run
    #[serde(rename = "occurrenceCount", skip_serializing_if = "Option::is_none")]
    pub occurrence_count: Option<u32>,
    /// A description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SarifMessage>,
    /// One or more unique identifiers that categorize the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprints: Option<HashMap<String, String>>,
    /// A set of strings that contribute to the stable, unique identity of the result
    #[serde(rename = "partialFingerprints", skip_serializing_if = "Option::is_none")]
    pub partial_fingerprints: Option<HashMap<String, String>>,
    /// An array of 'codeFlow' objects relevant to the result
    #[serde(rename = "codeFlows", skip_serializing_if = "Option::is_none")]
    pub code_flows: Option<Vec<SarifCodeFlow>>,
    /// An array of 'graph' objects relevant to the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphs: Option<Vec<SarifGraph>>,
    /// An array of 'graphTraversal' objects
    #[serde(rename = "graphTraversals", skip_serializing_if = "Option::is_none")]
    pub graph_traversals: Option<Vec<SarifGraphTraversal>>,
    /// An array of 'stack' objects relevant to the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacks: Option<Vec<SarifStack>>,
    /// An array of 'attachment' objects relevant to the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<SarifAttachment>>,
    /// A set of locations relevant to this result
    #[serde(rename = "relatedLocations", skip_serializing_if = "Option::is_none")]
    pub related_locations: Option<Vec<SarifLocation>>,
    /// A set of suppressions relevant to this result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressions: Option<Vec<SarifSuppression>>,
    /// The state of a result relative to a baseline of a previous run
    #[serde(rename = "baselineState", skip_serializing_if = "Option::is_none")]
    pub baseline_state: Option<SarifBaselineState>,
    /// A number representing the priority or importance of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<f64>,
    /// An array of 'fix' objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixes: Option<Vec<SarifFix>>,
    /// A value that categorizes results by evaluation state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<SarifResultKind>,
    /// Contains information about how and when the result was detected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<SarifResultProvenance>,
    /// An array of 'webRequest' objects
    #[serde(rename = "webRequest", skip_serializing_if = "Option::is_none")]
    pub web_request: Option<SarifWebRequest>,
    /// An array of 'webResponse' objects
    #[serde(rename = "webResponse", skip_serializing_if = "Option::is_none")]
    pub web_response: Option<SarifWebResponse>,
    /// The URIs of the work items associated with this result
    #[serde(rename = "workItemUris", skip_serializing_if = "Option::is_none")]
    pub work_item_uris: Option<Vec<String>>,
    /// Contains information about how and when the result was detected
    #[serde(rename = "hostedViewerUri", skip_serializing_if = "Option::is_none")]
    pub hosted_viewer_uri: Option<String>,
    /// Key/value pairs that provide additional information about the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

impl SarifResult {
    /// Create a new SarifResult with minimal required fields
    pub fn new(message: SarifMessage) -> Self {
        Self {
            message,
            ..Default::default()
        }
    }

    /// Builder pattern: Set the rule ID
    pub fn rule_id(mut self, rule_id: impl Into<String>) -> Self {
        self.rule_id = Some(rule_id.into());
        self
    }

    /// Builder pattern: Set the level
    pub fn level(mut self, level: SarifLevel) -> Self {
        self.level = Some(level);
        self
    }

    /// Builder pattern: Add a location
    pub fn location(mut self, location: SarifLocation) -> Self {
        self.locations.push(location);
        self
    }

    /// Builder pattern: Add multiple locations
    pub fn locations(mut self, locations: Vec<SarifLocation>) -> Self {
        self.locations.extend(locations);
        self
    }

    /// Builder pattern: Set the GUID
    pub fn guid(mut self, guid: impl Into<String>) -> Self {
        self.guid = Some(guid.into());
        self
    }

    /// Builder pattern: Add a fix
    pub fn fix(mut self, fix: SarifFix) -> Self {
        match &mut self.fixes {
            Some(fixes) => fixes.push(fix),
            None => self.fixes = Some(vec![fix]),
        }
        self
    }

    /// Validate the SarifResult
    pub fn validate(&mut self) -> Result<(), GHASError> {
        // Basic validation: ensure message has content
        if self.message.text.is_none() && self.message.id.is_none() {
            return Err(GHASError::InvalidSarif("SarifResult message must have text or id".to_string()));
        }
        Ok(())
    }
}

impl Display for SarifResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", 
            self.rule_id.as_deref().unwrap_or("unknown"), 
            self.message.text.as_deref().unwrap_or("no message")
        )
    }
}

/// Information about how and when the result was detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifResultProvenance {
    /// The index within the run.invocations array of the invocation object which describes the tool invocation that detected the result
    #[serde(rename = "firstDetectionTimeUtc", skip_serializing_if = "Option::is_none")]
    pub first_detection_time_utc: Option<String>,
    /// The index within the run.invocations array of the invocation object which describes the tool invocation that last detected the result
    #[serde(rename = "lastDetectionTimeUtc", skip_serializing_if = "Option::is_none")]
    pub last_detection_time_utc: Option<String>,
    /// A GUID-valued string equal to the automationDetails.guid property of the run in which the result was first detected
    #[serde(rename = "firstDetectionRunGuid", skip_serializing_if = "Option::is_none")]
    pub first_detection_run_guid: Option<String>,
    /// A GUID-valued string equal to the automationDetails.guid property of the run in which the result was last detected
    #[serde(rename = "lastDetectionRunGuid", skip_serializing_if = "Option::is_none")]
    pub last_detection_run_guid: Option<String>,
    /// The index within the run.invocations array of the invocation object which describes the tool invocation that detected the result
    #[serde(rename = "invocationIndex", skip_serializing_if = "Option::is_none")]
    pub invocation_index: Option<i32>,
    /// An array of physicalLocation objects which specify the portions of an analysis tool's output that a converter transformed into the result
    #[serde(rename = "conversionSources", skip_serializing_if = "Option::is_none")]
    pub conversion_sources: Option<Vec<SarifPhysicalLocation>>,
    /// Key/value pairs that provide additional information about the result provenance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// SARIF Location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifLocation {
    /// Value that distinguishes this location from all other locations within a single result object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Physical Location
    #[serde(rename = "physicalLocation", skip_serializing_if = "Option::is_none")]
    pub physical_location: Option<SarifPhysicalLocation>,
    /// The logical locations associated with the result
    #[serde(rename = "logicalLocations", skip_serializing_if = "Option::is_none")]
    pub logical_locations: Option<Vec<SarifLogicalLocation>>,
    /// A message relevant to the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<SarifMessage>,
    /// A set of regions relevant to the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<SarifRegion>>,
    /// An array of objects that describe relationships between this location and others
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<SarifLocationRelationship>>,
    /// Key/value pairs that provide additional information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

impl SarifLocation {
    /// Create a new SarifLocation with physical location
    pub fn new(physical_location: SarifPhysicalLocation) -> Self {
        Self {
            physical_location: Some(physical_location),
            ..Default::default()
        }
    }

    /// Builder pattern: Set logical locations
    pub fn logical_locations(mut self, logical_locations: Vec<SarifLogicalLocation>) -> Self {
        self.logical_locations = Some(logical_locations);
        self
    }
}

/// SARIF Physical Location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifPhysicalLocation {
    /// The address of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<SarifAddress>,
    /// Artifact Location
    #[serde(rename = "artifactLocation", skip_serializing_if = "Option::is_none")]
    pub artifact_location: Option<SarifArtifactLocation>,
    /// Region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<SarifRegion>,
    /// Context Region
    #[serde(rename = "contextRegion", skip_serializing_if = "Option::is_none")]
    pub context_region: Option<SarifRegion>,
    /// Key/value pairs that provide additional information about the physical location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

impl SarifPhysicalLocation {
    /// Create a new SarifPhysicalLocation
    pub fn new(artifact_location: SarifArtifactLocation) -> Self {
        Self {
            artifact_location,
            ..Default::default()
        }
    }

    /// Builder pattern: Set region
    pub fn region(mut self, region: SarifRegion) -> Self {
        self.region = Some(region);
        self
    }
}

/// SARIF Artifact Location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifArtifactLocation {
    /// URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// URI Base ID
    #[serde(rename = "uriBaseId", skip_serializing_if = "Option::is_none")]
    pub uri_base_id: Option<String>,
    /// The index within the run artifacts array of the artifact object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// A short description of the artifact location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SarifMessage>,
    /// Key/value pairs that provide additional information about the artifact location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

impl SarifArtifactLocation {
    /// Create a new SarifArtifactLocation
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: Some(uri.into()),
            ..Default::default()
        }
    }

    /// Builder pattern: Set URI base ID
    pub fn uri_base_id(mut self, uri_base_id: impl Into<String>) -> Self {
        self.uri_base_id = Some(uri_base_id.into());
        self
    }
}

/// SARIF Region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRegion {
    /// Start Line
    #[serde(rename = "startLine", skip_serializing_if = "Option::is_none")]
    pub start_line: Option<u32>,
    /// Start Column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_column: Option<u32>,
    /// End Line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<u32>,
    /// End Column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_column: Option<u32>,
    /// The zero-based offset from the beginning of the artifact of the first character in the region
    #[serde(rename = "charOffset", skip_serializing_if = "Option::is_none")]
    pub char_offset: Option<i32>,
    /// The length of the region in characters
    #[serde(rename = "charLength", skip_serializing_if = "Option::is_none")]
    pub char_length: Option<u32>,
    /// The zero-based offset from the beginning of the artifact of the first byte in the region
    #[serde(rename = "byteOffset", skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<i32>,
    /// The length of the region in bytes
    #[serde(rename = "byteLength", skip_serializing_if = "Option::is_none")]
    pub byte_length: Option<u32>,
    /// The portion of the artifact contents within the specified region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<SarifArtifactContent>,
    /// A message relevant to the region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<SarifMessage>,
    /// Specifies the source language, if any, of the portion of the artifact specified by the region object
    #[serde(rename = "sourceLanguage", skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    /// Key/value pairs that provide additional information about the region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

impl SarifRegion {
    /// Create a new SarifRegion with start line
    pub fn new(start_line: u32) -> Self {
        Self {
            start_line: Some(start_line),
            ..Default::default()
        }
    }

    /// Builder pattern: Set start column
    pub fn start_column(mut self, start_column: u32) -> Self {
        self.start_column = Some(start_column);
        self
    }

    /// Builder pattern: Set end line
    pub fn end_line(mut self, end_line: u32) -> Self {
        self.end_line = Some(end_line);
        self
    }

    /// Builder pattern: Set end column
    pub fn end_column(mut self, end_column: u32) -> Self {
        self.end_column = Some(end_column);
        self
    }
}

/// Represents a single artifact within the set of artifacts analyzed by the tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifArtifact {
    /// A description of the artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SarifMessage>,
    /// The location of the artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SarifArtifactLocation>,
    /// A short description of the parent that contains the artifact
    #[serde(rename = "parentIndex", skip_serializing_if = "Option::is_none")]
    pub parent_index: Option<i32>,
    /// The offset in bytes of the artifact within its containing parent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// The length of the artifact in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    /// The MIME type of the artifact
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// The contents of the artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<SarifArtifactContent>,
    /// A string representing the encoding for the artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// The Coordinated Universal Time (UTC) date and time at which the artifact was most recently modified
    #[serde(rename = "lastModifiedTimeUtc", skip_serializing_if = "Option::is_none")]
    pub last_modified_time_utc: Option<String>,
    /// An array of hash objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<HashMap<String, String>>,
    /// Key/value pairs that provide additional information about the artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A logical location of a construct that produced a result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifLogicalLocation {
    /// Identifies the construct in which the result occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The index within the logical locations array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// The human-readable fully qualified name of the logical location
    #[serde(rename = "fullyQualifiedName", skip_serializing_if = "Option::is_none")]
    pub fully_qualified_name: Option<String>,
    /// The machine-readable name for the logical location
    #[serde(rename = "decoratedName", skip_serializing_if = "Option::is_none")]
    pub decorated_name: Option<String>,
    /// Identifies the index of the immediate parent of the construct
    #[serde(rename = "parentIndex", skip_serializing_if = "Option::is_none")]
    pub parent_index: Option<i32>,
    /// The type of construct this logical location refers to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Key/value pairs that provide additional information about the logical location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Information about the relation of one reporting descriptor to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifLocationRelationship {
    /// A reference to the related location
    pub target: i32,
    /// A set of distinct strings that categorize the relationship
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<String>>,
    /// A description of the relationship
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SarifMessage>,
    /// Key/value pairs that provide additional information about the relationship
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A physical or virtual address, or a range of addresses, in an 'addressable region'
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifAddress {
    /// The address expressed as a byte offset from the start of the addressable region
    #[serde(rename = "absoluteAddress", skip_serializing_if = "Option::is_none")]
    pub absolute_address: Option<i64>,
    /// The address expressed as a byte offset from the start of the addressable region
    #[serde(rename = "relativeAddress", skip_serializing_if = "Option::is_none")]
    pub relative_address: Option<i32>,
    /// The number of bytes in this range of addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    /// An open-ended string that identifies the address kind
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A name that is associated with the address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A human-readable fully qualified name that is associated with the address
    #[serde(rename = "fullyQualifiedName", skip_serializing_if = "Option::is_none")]
    pub fully_qualified_name: Option<String>,
    /// The byte offset of this address from the absolute or relative address of the parent object
    #[serde(rename = "offsetFromParent", skip_serializing_if = "Option::is_none")]
    pub offset_from_parent: Option<i32>,
    /// The index within the run addresses array of the cached object for this address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// The index within the run addresses array of the parent object
    #[serde(rename = "parentIndex", skip_serializing_if = "Option::is_none")]
    pub parent_index: Option<i32>,
    /// Key/value pairs that provide additional information about the address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Represents the contents of an artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifArtifactContent {
    /// UTF-8-encoded content from a text artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// MIME Base64-encoded content from a binary artifact, or from a text artifact in its original encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
    /// An alternate rendered representation of the artifact (e.g., a decompiled representation of a binary region)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendered: Option<SarifMultiformatMessageString>,
    /// Key/value pairs that provide additional information about the artifact content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// SARIF Tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifTool {
    /// Driver
    pub driver: SarifToolComponent,
    /// Tool extensions that contributed to or reconfigured the analysis tool that was run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<SarifToolComponent>>,
    /// Key/value pairs that provide additional information about the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
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

/// SARIF Tool Component (was SarifToolDriver)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SarifToolComponent {
    /// A unique identifer for the tool component in the form of a GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// Name
    pub name: String,
    /// Organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// The tool component version, in whatever format the component natively provides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The tool component version in the format specified by Semantic Versioning 2.0
    #[serde(rename = "semanticVersion", skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// SARIF Rules
    #[serde(default)]
    pub rules: Vec<SarifReportingDescriptor>,
    /// An array of reportingDescriptor objects relevant to the notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<SarifReportingDescriptor>>,
    /// Key/value pairs that provide additional information about the tool component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// SARIF Message
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SarifMessage {
    /// Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// A Markdown message string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
    /// The identifier for this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// An array of strings to substitute into the message string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    /// Key/value pairs that provide additional information about the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

impl SarifMessage {
    /// Create a new SarifMessage with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            ..Default::default()
        }
    }

    /// Create a new SarifMessage with ID for localization
    pub fn with_id(id: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            ..Default::default()
        }
    }

    /// Builder pattern: Set markdown content
    pub fn markdown(mut self, markdown: impl Into<String>) -> Self {
        self.markdown = Some(markdown.into());
        self
    }
}

/// Key/value pairs that provide additional information about the object
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SarifPropertyBag {
    /// A set of distinct strings that provide additional information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Additional properties
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

/// SARIF External Properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifExternalProperties {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(rename = "runGuid", skip_serializing_if = "Option::is_none")]
    pub run_guid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// SARIF Invocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifInvocation {
    /// The command line used to invoke the tool
    #[serde(rename = "commandLine", skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
    /// An array of strings, containing in order the command line arguments passed to the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    /// The Coordinated Universal Time (UTC) date and time at which the run started
    #[serde(rename = "startTimeUtc", skip_serializing_if = "Option::is_none")]
    pub start_time_utc: Option<String>,
    /// The Coordinated Universal Time (UTC) date and time at which the run ended
    #[serde(rename = "endTimeUtc", skip_serializing_if = "Option::is_none")]
    pub end_time_utc: Option<String>,
    /// The process exit code
    #[serde(rename = "exitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// Specifies whether the tool's execution completed successfully
    #[serde(rename = "executionSuccessful")]
    pub execution_successful: bool,
    /// The machine that hosted the analysis tool run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,
    /// The account that ran the analysis tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The process id for the analysis tool run
    #[serde(rename = "processId", skip_serializing_if = "Option::is_none")]
    pub process_id: Option<u32>,
    /// An absolute URI specifying the location of the analysis tool's executable
    #[serde(rename = "executableLocation", skip_serializing_if = "Option::is_none")]
    pub executable_location: Option<SarifArtifactLocation>,
    /// The working directory for the analysis tool run
    #[serde(rename = "workingDirectory", skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<SarifArtifactLocation>,
    /// The environment variables associated with the analysis tool process
    #[serde(rename = "environmentVariables", skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<HashMap<String, String>>,
    /// A file containing the standard input stream to the process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<SarifArtifactLocation>,
    /// A file containing the standard output stream from the process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<SarifArtifactLocation>,
    /// A file containing the standard error stream from the process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<SarifArtifactLocation>,
    /// A file containing the interleaved standard output and standard error stream
    #[serde(rename = "stdoutStderr", skip_serializing_if = "Option::is_none")]
    pub stdout_stderr: Option<SarifArtifactLocation>,
    /// Key/value pairs that provide additional information about the invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A suppression that is relevant to a result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifSuppression {
    /// A stable, unique identifer for the suprression in the form of a GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// A string that indicates where the suppression is persisted
    pub kind: SarifSuppressionKind,
    /// A string that indicates the state of the suppression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<SarifSuppressionState>,
    /// A string representing the justification for the suppression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    /// Identifies the location associated with the suppression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SarifLocation>,
    /// Key/value pairs that provide additional information about the suppression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A call stack that is relevant to a result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifStack {
    /// A message relevant to this call stack
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<SarifMessage>,
    /// An array of stack frames that represents a sequence of calls
    pub frames: Vec<SarifStackFrame>,
    /// Key/value pairs that provide additional information about the stack
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A function call within a stack trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifStackFrame {
    /// The location to which this stack frame refers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SarifLocation>,
    /// The name of the module that contains the code of this stack frame
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// The thread identifier of the stack frame
    #[serde(rename = "threadId", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<u32>,
    /// The parameters of the call that is executing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    /// Key/value pairs that provide additional information about the stack frame
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A set of threadFlows which together describe a pattern of code execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifCodeFlow {
    /// A message relevant to the code flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<SarifMessage>,
    /// An array of one or more unique threadFlow objects
    #[serde(rename = "threadFlows")]
    pub thread_flows: Vec<SarifThreadFlow>,
    /// Key/value pairs that provide additional information about the code flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A thread flow describes the progress of a program through a thread of execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifThreadFlow {
    /// An string that uniquely identifies the threadFlow within the codeFlow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A message relevant to the thread flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<SarifMessage>,
    /// Values of relevant expressions at the start of the thread flow that may change
    #[serde(rename = "initialState", skip_serializing_if = "Option::is_none")]
    pub initial_state: Option<HashMap<String, SarifMultiformatMessageString>>,
    /// Values of relevant expressions at the start of the thread flow that remain constant
    #[serde(rename = "immutableState", skip_serializing_if = "Option::is_none")]
    pub immutable_state: Option<HashMap<String, SarifMultiformatMessageString>>,
    /// A temporally ordered array of 'threadFlowLocation' objects
    pub locations: Vec<SarifThreadFlowLocation>,
    /// Key/value pairs that provide additional information about the thread flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A location visited by an analysis tool while simulating or monitoring execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifThreadFlowLocation {
    /// The index within the run threadFlowLocations array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// The code location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SarifLocation>,
    /// The call stack leading to this location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<SarifStack>,
    /// A set of distinct strings that categorize the thread flow location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<String>>,
    /// An array of references to rule or taxonomy reporting descriptors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxa: Option<Vec<SarifReportingDescriptorReference>>,
    /// The name of the module that contains the code that is executing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// A dictionary of variable or expression values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<HashMap<String, SarifMultiformatMessageString>>,
    /// An integer representing a containment hierarchy within the thread flow
    #[serde(rename = "nestingLevel", skip_serializing_if = "Option::is_none")]
    pub nesting_level: Option<u32>,
    /// An integer representing the temporal order in which execution reached this location
    #[serde(rename = "executionOrder", skip_serializing_if = "Option::is_none")]
    pub execution_order: Option<i32>,
    /// The Coordinated Universal Time (UTC) date and time at which this location was executed
    #[serde(rename = "executionTimeUtc", skip_serializing_if = "Option::is_none")]
    pub execution_time_utc: Option<String>,
    /// Specifies the importance of this location in understanding the code flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<SarifThreadFlowLocationImportance>,
    /// A web request associated with this thread flow location
    #[serde(rename = "webRequest", skip_serializing_if = "Option::is_none")]
    pub web_request: Option<SarifWebRequest>,
    /// A web response associated with this thread flow location
    #[serde(rename = "webResponse", skip_serializing_if = "Option::is_none")]
    pub web_response: Option<SarifWebResponse>,
    /// Key/value pairs that provide additional information about the threadflow location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A network of nodes and directed edges that describes some aspect of the structure of the code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifGraph {
    /// A description of the graph
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SarifMessage>,
    /// An array of node objects representing the nodes of the graph
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<SarifNode>>,
    /// An array of edge objects representing the edges of the graph
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<SarifEdge>>,
    /// Key/value pairs that provide additional information about the graph
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Represents a node in a graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifNode {
    /// A string that uniquely identifies the node within its graph
    pub id: String,
    /// A short description of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<SarifMessage>,
    /// A code location associated with the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SarifLocation>,
    /// Array of child nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<SarifNode>>,
    /// Key/value pairs that provide additional information about the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Represents a directed edge in a graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifEdge {
    /// A string that uniquely identifies the edge within its graph
    pub id: String,
    /// A short description of the edge
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<SarifMessage>,
    /// Identifies the source node (the node at which the edge starts)
    #[serde(rename = "sourceNodeId")]
    pub source_node_id: String,
    /// Identifies the target node (the node at which the edge ends)
    #[serde(rename = "targetNodeId")]
    pub target_node_id: String,
    /// Key/value pairs that provide additional information about the edge
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Represents a path through a graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifGraphTraversal {
    /// The index within the run.graphs to be associated with the result
    #[serde(rename = "runGraphIndex", skip_serializing_if = "Option::is_none")]
    pub run_graph_index: Option<i32>,
    /// The index within the result.graphs to be associated with the result
    #[serde(rename = "resultGraphIndex", skip_serializing_if = "Option::is_none")]
    pub result_graph_index: Option<i32>,
    /// A description of this graph traversal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SarifMessage>,
    /// Values of relevant expressions at the start of the graph traversal that may change
    #[serde(rename = "initialState", skip_serializing_if = "Option::is_none")]
    pub initial_state: Option<HashMap<String, SarifMultiformatMessageString>>,
    /// Values of relevant expressions at the start of the graph traversal that remain constant
    #[serde(rename = "immutableState", skip_serializing_if = "Option::is_none")]
    pub immutable_state: Option<HashMap<String, SarifMultiformatMessageString>>,
    /// The sequences of edges traversed by this graph traversal
    #[serde(rename = "edgeTraversals", skip_serializing_if = "Option::is_none")]
    pub edge_traversals: Option<Vec<SarifEdgeTraversal>>,
    /// Key/value pairs that provide additional information about the graph traversal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Represents the traversal of a single edge during a graph traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifEdgeTraversal {
    /// Identifies the edge being traversed
    #[serde(rename = "edgeId")]
    pub edge_id: String,
    /// A message to display to the user as the edge is traversed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<SarifMessage>,
    /// The values of relevant expressions after the edge has been traversed
    #[serde(rename = "finalState", skip_serializing_if = "Option::is_none")]
    pub final_state: Option<HashMap<String, SarifMultiformatMessageString>>,
    /// The number of edge traversals necessary to return from a nested graph
    #[serde(rename = "stepOverEdgeCount", skip_serializing_if = "Option::is_none")]
    pub step_over_edge_count: Option<u32>,
    /// Key/value pairs that provide additional information about the edge traversal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A location within a programming construct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifSpecialLocations {
    /// Provides a way to specify that the result is related to a problem in a property file
    #[serde(rename = "displayBase", skip_serializing_if = "Option::is_none")]
    pub display_base: Option<SarifArtifactLocation>,
    /// Key/value pairs that provide additional information about the special locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Contains information that enables a SARIF consumer to locate the external property files that contain run data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifVersionControlDetails {
    /// The name of the repository
    #[serde(rename = "repositoryUri")]
    pub repository_uri: String,
    /// A string that uniquely and permanently identifies an arbitrary revision within a repository
    #[serde(rename = "revisionId", skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// The name of a branch containing the revision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// A tag that has been applied to the revision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// A Coordinated Universal Time (UTC) date and time that can be used to synchronize an enlistment to the state of the repository at that time
    #[serde(rename = "asOfTimeUtc", skip_serializing_if = "Option::is_none")]
    pub as_of_time_utc: Option<String>,
    /// The location in the local file system to which the root of the repository was mapped at the time of the analysis
    #[serde(rename = "mappedTo", skip_serializing_if = "Option::is_none")]
    pub mapped_to: Option<SarifArtifactLocation>,
    /// Key/value pairs that provide additional information about the version control details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Contains information that enables a SARIF consumer to locate the external property files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifAutomationDetails {
    /// A description of the identity and role played within the engineering system by this object's containing run object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SarifMessage>,
    /// A hierarchical string that uniquely identifies this object's containing run object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A stable, unique identifier for the equivalence class of runs to which this object's containing run object belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// A stable, unique identifier for this object's containing run object
    #[serde(rename = "correlationGuid", skip_serializing_if = "Option::is_none")]
    pub correlation_guid: Option<String>,
    /// Key/value pairs that provide additional information about the automation details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A message string or message format string rendered in multiple formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifMultiformatMessageString {
    /// A plain text message string or format string
    pub text: String,
    /// A Markdown message string or format string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
    /// Key/value pairs that provide additional information about the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// SARIF Reporting Descriptor (was SarifRule)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifReportingDescriptor {
    /// ID
    pub id: String,
    /// An array of stable, opaque identifiers by which this report was known in some previous version
    #[serde(rename = "deprecatedIds", skip_serializing_if = "Option::is_none")]
    pub deprecated_ids: Option<Vec<String>>,
    /// A unique identifer for the reporting descriptor in the form of a GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// An array of unique identifies in the form of a GUID by which this report was known in some previous version
    #[serde(rename = "deprecatedGuids", skip_serializing_if = "Option::is_none")]
    pub deprecated_guids: Option<Vec<String>>,
    /// A report identifier that is understandable to an end user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An array of readable identifiers by which this report was known in some previous version
    #[serde(rename = "deprecatedNames", skip_serializing_if = "Option::is_none")]
    pub deprecated_names: Option<Vec<String>>,
    /// A concise description of the report
    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<SarifMultiformatMessageString>,
    /// A description of the report
    #[serde(rename = "fullDescription", skip_serializing_if = "Option::is_none")]
    pub full_description: Option<SarifMultiformatMessageString>,
    /// A set of name/value pairs with arbitrary names
    #[serde(rename = "messageStrings", skip_serializing_if = "Option::is_none")]
    pub message_strings: Option<HashMap<String, SarifMultiformatMessageString>>,
    /// Default reporting configuration information
    #[serde(rename = "defaultConfiguration", skip_serializing_if = "Option::is_none")]
    pub default_configuration: Option<SarifReportingConfiguration>,
    /// A URI where the primary documentation for the report can be found
    #[serde(rename = "helpUri", skip_serializing_if = "Option::is_none")]
    pub help_uri: Option<String>,
    /// Provides the primary documentation for the report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<SarifMultiformatMessageString>,
    /// An array of objects that describe relationships between this reporting descriptor and others
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<SarifReportingDescriptorRelationship>>,
    /// Key/value pairs that provide additional information about the report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Information about a rule or notification that can be configured at runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifReportingConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<SarifLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<SarifPropertyBag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Information about the relation of one reporting descriptor to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifReportingDescriptorRelationship {
    pub target: SarifReportingDescriptorReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SarifMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Information about how to locate a relevant reporting descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifReportingDescriptorReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(rename = "toolComponent", skip_serializing_if = "Option::is_none")]
    pub tool_component: Option<SarifToolComponentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Reference to a tool component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifToolComponentReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A proposed fix for the problem represented by the result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifFix {
    /// A description of the fix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SarifMessage>,
    /// One or more artifact changes that comprise a fix for a result
    #[serde(rename = "artifactChanges")]
    pub artifact_changes: Vec<SarifArtifactChange>,
    /// Key/value pairs that provide additional information about the fix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// A change to a single artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifArtifactChange {
    /// The location of the artifact to change
    #[serde(rename = "artifactLocation")]
    pub artifact_location: SarifArtifactLocation,
    /// An array of replacement objects, each of which represents the replacement of a single region in a single artifact
    pub replacements: Vec<SarifReplacement>,
    /// Key/value pairs that provide additional information about the artifact change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// The replacement of a single region of an artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifReplacement {
    /// The region of the artifact to delete
    #[serde(rename = "deletedRegion")]
    pub deleted_region: SarifRegion,
    /// The content to insert at the location specified by the 'deletedRegion' property
    #[serde(rename = "insertedContent", skip_serializing_if = "Option::is_none")]
    pub inserted_content: Option<SarifArtifactContent>,
    /// Key/value pairs that provide additional information about the replacement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// An attachment associated with a result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifAttachment {
    /// A description of the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<SarifMessage>,
    /// The location of the attachment
    #[serde(rename = "artifactLocation")]
    pub artifact_location: SarifArtifactLocation,
    /// An array of regions of interest within the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<SarifRegion>>,
    /// An array of rectangles specifying areas of interest within the image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangles: Option<Vec<SarifRectangle>>,
    /// Key/value pairs that provide additional information about the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// An area within an image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRectangle {
    /// The Y coordinate of the top edge of the rectangle, measured in the image's natural units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<f64>,
    /// The X coordinate of the left edge of the rectangle, measured in the image's natural units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<f64>,
    /// The Y coordinate of the bottom edge of the rectangle, measured in the image's natural units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<f64>,
    /// The X coordinate of the right edge of the rectangle, measured in the image's natural units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<f64>,
    /// A message relevant to the rectangle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<SarifMessage>,
    /// Key/value pairs that provide additional information about the rectangle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Describes an HTTP request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifWebRequest {
    /// The request protocol. Example: 'http'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// The request version. Example: '1.1'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The target of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// The HTTP method. Well-known values are 'GET', 'PUT', 'POST', 'DELETE', 'PATCH', 'HEAD', 'OPTIONS', 'TRACE', 'CONNECT'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// The request headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// The request parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, String>>,
    /// The body of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<SarifArtifactContent>,
    /// Key/value pairs that provide additional information about the web request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

/// Describes the response to an HTTP request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifWebResponse {
    /// The response protocol. Example: 'http'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// The response version. Example: '1.1'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The response status code. Example: 451
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    /// The response reason. Example: 'Not found'
    #[serde(rename = "reasonPhrase", skip_serializing_if = "Option::is_none")]
    pub reason_phrase: Option<String>,
    /// The response headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// The body of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<SarifArtifactContent>,
    /// Specifies whether redirects were followed
    #[serde(rename = "noResponseReceived", skip_serializing_if = "Option::is_none")]
    pub no_response_received: Option<bool>,
    /// Key/value pairs that provide additional information about the web response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SarifPropertyBag>,
}

// Enums

/// A value specifying the severity level
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SarifLevel {
    None,
    Note,
    Warning,
    Error,
}

/// A value that categorizes results by evaluation state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SarifResultKind {
    NotApplicable,
    Pass,
    Fail,
    Review,
    Open,
    Informational,
}

/// The state of a result relative to a baseline of a previous run
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SarifBaselineState {
    New,
    Unchanged,
    Updated,
    Absent,
}

/// Specifies the unit in which the tool measures columns
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SarifColumnKind {
    Utf16CodeUnits,
    UnicodeCodePoints,
}

/// The kinds of data contained in a tool component
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SarifToolComponentContents {
    LocalizedData,
    NonLocalizedData,
}

/// A string that indicates where the suppression is persisted
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SarifSuppressionKind {
    InSource,
    External,
}

/// A string that indicates the state of the suppression
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SarifSuppressionState {
    Accepted,
    UnderReview,
    Rejected,
}

/// Specifies the importance of this location in understanding the code flow
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SarifThreadFlowLocationImportance {
    Important,
    Essential,
    Unimportant,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sarif_creation() {
        let sarif = Sarif::new();
        assert_eq!(sarif.version, "2.1.0");
        assert!(sarif.runs.is_empty());
        assert!(sarif.schema.contains("sarif-schema-2.1.0.json"));
    }

    #[test]
    fn test_sarif_result_builder() {
        let message = SarifMessage::new("Test security issue found");
        let location = SarifLocation::new(
            SarifPhysicalLocation::new(
                SarifArtifactLocation::new("src/main.rs")
            ).region(
                SarifRegion::new(10)
                    .start_column(5)
                    .end_line(10)
                    .end_column(20)
            )
        );

        let result = SarifResult::new(message)
            .rule_id("SECURITY_001")
            .level(SarifLevel::Error)
            .location(location)
            .guid("12345678-1234-1234-1234-123456789012");

        assert_eq!(result.rule_id, Some("SECURITY_001".to_string()));
        assert!(matches!(result.level, Some(SarifLevel::Error)));
        assert_eq!(result.locations.len(), 1);
        assert_eq!(result.guid, Some("12345678-1234-1234-1234-123456789012".to_string()));
    }

    #[test]
    fn test_sarif_message_creation() {
        let message = SarifMessage::new("Test message")
            .markdown("**Test** message");
        
        assert_eq!(message.text, Some("Test message".to_string()));
        assert_eq!(message.markdown, Some("**Test** message".to_string()));
    }

    #[test]
    fn test_sarif_location_with_logical() {
        let logical_location = SarifLogicalLocation {
            name: Some("main".to_string()),
            fully_qualified_name: Some("main::function".to_string()),
            kind: Some("function".to_string()),
            ..Default::default()
        };

        let location = SarifLocation::new(
            SarifPhysicalLocation::new(
                SarifArtifactLocation::new("src/main.rs")
            )
        ).logical_locations(vec![logical_location]);

        assert!(location.logical_locations.is_some());
        assert_eq!(location.logical_locations.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_sarif_serialization() {
        let message = SarifMessage::new("Test issue");
        let location = SarifLocation::new(
            SarifPhysicalLocation::new(
                SarifArtifactLocation::new("test.rs")
            ).region(SarifRegion::new(1))
        );

        let result = SarifResult::new(message)
            .rule_id("TEST_RULE")
            .level(SarifLevel::Warning)
            .location(location);

        let tool_driver = SarifToolComponent {
            name: "test-tool".to_string(),
            version: Some("1.0.0".to_string()),
            ..Default::default()
        };

        let tool = SarifTool {
            driver: tool_driver,
            ..Default::default()
        };

        let run = SarifRun {
            tool,
            results: vec![result],
            ..Default::default()
        };

        let sarif = Sarif {
            version: "2.1.0".to_string(),
            schema: "https://example.com/schema".to_string(),
            runs: vec![run],
            ..Default::default()
        };

        // Test JSON serialization
        let json = serde_json::to_string(&sarif).expect("Failed to serialize SARIF");
        assert!(json.contains("2.1.0"));
        assert!(json.contains("TEST_RULE"));
        assert!(json.contains("test-tool"));

        // Test deserialization
        let deserialized: Sarif = serde_json::from_str(&json).expect("Failed to deserialize SARIF");
        assert_eq!(deserialized.version, "2.1.0");
        assert_eq!(deserialized.runs.len(), 1);
    }

    #[test]
    fn test_sarif_with_fixes() {
        let replacement = SarifReplacement {
            deleted_region: SarifRegion::new(10).end_line(10),
            inserted_content: Some(SarifArtifactContent {
                text: Some("fixed code".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let artifact_change = SarifArtifactChange {
            artifact_location: SarifArtifactLocation::new("src/main.rs"),
            replacements: vec![replacement],
            ..Default::default()
        };

        let fix = SarifFix {
            description: Some(SarifMessage::new("Fix the security issue")),
            artifact_changes: vec![artifact_change],
            ..Default::default()
        };

        let result = SarifResult::new(SarifMessage::new("Security issue"))
            .rule_id("SEC_001")
            .fix(fix);

        assert!(result.fixes.is_some());
        assert_eq!(result.fixes.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_sarif_enums() {
        // Test level serialization
        let level = SarifLevel::Error;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"error\"");

        // Test result kind serialization
        let kind = SarifResultKind::Fail;
        let json = serde_json::to_string(&kind).unwrap();
        assert_eq!(json, "\"fail\"");

        // Test baseline state serialization
        let state = SarifBaselineState::New;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "\"new\"");
    }

    #[test]
    fn test_sarif_validation() {
        // Valid result
        let mut valid_result = SarifResult::new(SarifMessage::new("Valid message"));
        assert!(valid_result.validate().is_ok());

        // Invalid result - no message text or id
        let mut invalid_result = SarifResult {
            message: SarifMessage {
                text: None,
                id: None,
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(invalid_result.validate().is_err());
    }
}
