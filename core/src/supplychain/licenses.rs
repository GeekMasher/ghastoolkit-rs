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
}

impl Iterator for Licenses {
    type Item = License;

    fn next(&mut self) -> Option<Self::Item> {
        self.licenses.pop()
    }
}

impl From<&str> for Licenses {
    fn from(value: &str) -> Self {
        let lowered = value.to_lowercase();
        let mut licenses = Licenses::new();

        for license in lowered.split("and") {
            licenses.push(License::from(license.trim()));
        }
        licenses
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
    }
}
