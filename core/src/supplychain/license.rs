use serde::{Deserialize, Serialize};

/// A Dependency License enum with SPDX and custom licenses. We only support a few licenses
/// but you can use the `Custom` variant to add your own license.
/// SPDX License List: https://spdx.org/licenses/
///
/// # Example
///
/// ```rust
/// use ghastoolkit::supplychain::License;
///
/// let license = License::from("MIT");
/// assert_eq!(license, License::MIT);
///
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum License {
    /// Apache 2.0
    Apache2,
    /// MIT
    MIT,
    /// GPL 3.0
    GPL3,
    /// LGPL 3.0
    LGPL3,
    /// AGPL 3.0
    AGPL3,
    /// MPL 2.0
    MPL2,
    /// BSD 2-Clause
    BSD2,
    /// BSD 3-Clause
    BSD3,
    /// CC0
    CC0,
    /// ISC
    ISC,
    /// Custom license
    Custom(String),
    /// Unknown license
    #[default]
    Unknown,
}

impl From<&str> for License {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "apache-2.0" | "apache 2.0" => License::Apache2,
            "mit" | "mit license" => License::MIT,
            "gpl-3.0" | "gpl 3.0" => License::GPL3,
            "lgpl-3.0" | "lgpl 3.0" => License::LGPL3,
            "agpl-3.0" | "agpl 3.0" => License::AGPL3,
            "mpl-2.0" | "mpl 2.0" => License::MPL2,
            "bsd-2-clause" | "bsd 2-clause" => License::BSD2,
            "bsd-3-clause" | "bsd 3-clause" => License::BSD3,
            "cc0" => License::CC0,
            "isc" => License::ISC,
            _ => License::Custom(String::from(value)),
        }
    }
}

impl From<String> for License {
    fn from(value: String) -> Self {
        License::from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::License;

    #[test]
    fn test_license_from_str() {
        let license = License::from("Apache 2.0");
        assert_eq!(license, License::Apache2);
    }
}
