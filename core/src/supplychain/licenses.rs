use serde::{Deserialize, Serialize};

use crate::supplychain::License;

/// List of Licenses for a dependency
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Licenses {
    licenses: Vec<License>,
}

impl Licenses {
    /// Create a new list of licenses
    pub fn new() -> Self {
        Self {
            licenses: Vec::new(),
        }
    }

    /// Push a new license to the list
    pub fn push(&mut self, license: License) {
        self.licenses.push(license);
    }

    /// Check if the list of licenses is empty
    pub fn is_empty(&self) -> bool {
        self.licenses.is_empty()
    }

    /// Get the length of the list of licenses
    pub fn len(&self) -> usize {
        self.licenses.len()
    }

    /// Parse a string into a list of licenses
    pub fn parse(value: &str) -> Licenses {
        match value.to_lowercase().as_str() {
            value if value.contains("and") => Licenses::parse_sep(value, "and"),
            value if value.contains(',') => Licenses::parse_sep(value, ","),
            _ => Licenses::new(),
        }
    }

    fn parse_sep(value: &str, sep: &str) -> Licenses {
        let mut licenses = Licenses::new();
        for license in value.split(sep) {
            licenses.push(License::from(license.trim()));
        }
        licenses
    }
}

impl Iterator for Licenses {
    type Item = License;

    fn next(&mut self) -> Option<Self::Item> {
        self.licenses.pop()
    }
}

impl From<&str> for Licenses {
    fn from(value: &str) -> Self {
        Licenses::parse(value)
    }
}

impl From<String> for Licenses {
    fn from(value: String) -> Self {
        Licenses::parse(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::supplychain::{License, Licenses};

    #[test]
    fn test_licenses_from_str() {
        let licenses = Licenses::from("Apache-2.0 AND MIT");

        let correct = Licenses {
            licenses: vec![License::Apache(String::from("2.0")), License::MIT],
        };

        assert_eq!(licenses, correct);
        assert_eq!(licenses.len(), 2);
    }

    #[test]
    fn test_licenses_commasep() {
        let licenses = Licenses::from("Apache-2.0, MIT");

        let correct = Licenses {
            licenses: vec![License::Apache(String::from("2.0")), License::MIT],
        };

        assert_eq!(licenses, correct);
    }
}
