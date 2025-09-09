use std::collections::HashMap;

/// JSON representation of the languages supported by the CodeQL CLI
///
/// ```bash
/// codeql resolve languages --format json
/// ```
pub(crate) type ResolvedLanguages = HashMap<String, Vec<String>>;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct ResolvedPacks {
    pub steps: Vec<ResolvedPackStep>
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct ResolvedPackStep {
    pub r#type: String,
    #[serde(default)]
    pub found: HashMap<String, HashMap<String, ResolvedPackInfo>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct ResolvedPackInfo {
    pub kind: String,
    pub path: String,   
}
