use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// GitHub Message block
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Message {
    /// The message text
    pub text: String,
}

/// GItHub Source Location
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Location {
    /// Path
    pub path: String,
    /// Start Line
    pub start_line: u32,
    /// End Line
    pub end_line: u32,
    /// Start Column
    pub start_column: u32,
    /// End Column
    pub end_column: u32,
}

/// GitHub Languages
pub type GitHubLanguages = HashMap<String, u32>;
