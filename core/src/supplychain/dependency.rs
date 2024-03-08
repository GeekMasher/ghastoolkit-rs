use std::{collections::HashMap, fmt::Display, str::FromStr};

use purl::GenericPurl;

use crate::{supplychain::licenses::Licenses, Repository};

/// Supply Chain Dependency struct used to represent a dependency in a supply chain.
///
/// # Example
///
/// ```rust
/// use ghastoolkit::Dependency;
/// // Create a new Dependency from a PURL
/// let dependency = Dependency::from("pkg:generic/namespace/name@version");
///
/// println!("{}", dependency);
///
/// ```
#[derive(Debug, Clone, Default)]
pub struct Dependency {
    /// Manager / Type of the dependency
    pub manager: String,
    /// Name of the dependency
    pub name: String,
    /// Namespace of the dependency
    pub namespace: Option<String>,
    /// Version of the dependency
    pub version: Option<String>,
    /// Path to the dependency
    path: Option<String>,
    /// Qualifiers for the dependency
    qualifiers: HashMap<String, String>,
    /// SPDX licenses for the dependency
    pub license: Licenses,

    repository: Option<Repository>,
    /// PURL
    purl: Option<GenericPurl<String>>,
}

impl Dependency {
    /// Create a new Dependency
    pub fn new() -> Self {
        Default::default()
    }

    /// Get the PURL for the dependency
    pub fn purl(&self) -> String {
        if let Some(purl) = &self.purl {
            purl.to_string()
        } else {
            format!("pkg:{}@{}", self.manager, self.name)
        }
    }
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.purl())
    }
}

impl From<&str> for Dependency {
    fn from(value: &str) -> Self {
        Dependency::from(GenericPurl::<String>::from_str(value).expect("Failed to parse PURL"))
    }
}

impl From<String> for Dependency {
    fn from(value: String) -> Self {
        Dependency::from(value.as_str())
    }
}

impl From<GenericPurl<String>> for Dependency {
    fn from(value: GenericPurl<String>) -> Self {
        Dependency {
            name: value.name().to_string(),
            namespace: value.namespace().map(|s| s.to_string()),
            version: value.version().map(|s| s.to_string()),
            manager: value.package_type().clone(),
            purl: Some(value),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_from_str() {
        let dependency = Dependency::from("pkg:generic/namespace/name@version");
        assert_eq!(dependency.name, "name");
        assert_eq!(dependency.namespace, Some("namespace".to_string()));
        assert_eq!(dependency.version, Some("version".to_string()));
        assert_eq!(dependency.manager, "generic".to_string());

        assert_eq!(
            dependency.purl(),
            "pkg:generic/namespace/name@version".to_string()
        );
    }
}
