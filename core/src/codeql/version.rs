//! CodeQL Version / Release

use std::fmt::Display;

/// CodeQL Version
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum CodeQLVersion {
    /// Latest CodeQL Version
    #[default]
    Latest,
    /// Nightly version
    Nightly,
    /// Specific version of CodeQL
    Version(String),
}

impl Display for CodeQLVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeQLVersion::Latest => write!(f, "latest"),
            CodeQLVersion::Nightly => write!(f, "nightly"),
            CodeQLVersion::Version(v) => write!(f, "{v}"),
        }
    }
}

impl From<&str> for CodeQLVersion {
    fn from(value: &str) -> Self {
        match value {
            "latest" | "stable" => CodeQLVersion::Latest,
            "nightly" | "unstable" => CodeQLVersion::Nightly,
            v => CodeQLVersion::Version(v.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codeql_version_from() {
        assert_eq!(CodeQLVersion::from("latest"), CodeQLVersion::Latest);
        assert_eq!(CodeQLVersion::from("stable"), CodeQLVersion::Latest);
        assert_eq!(CodeQLVersion::from("nightly"), CodeQLVersion::Nightly);
        assert_eq!(CodeQLVersion::from("unstable"), CodeQLVersion::Nightly);
        assert_eq!(
            CodeQLVersion::from("2.7.5"),
            CodeQLVersion::Version("2.7.5".to_string())
        );
    }
}
