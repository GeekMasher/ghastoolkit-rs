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
    /// Apache (1.0, 1.1, 2.0)
    Apache(String),
    /// MIT
    MIT,
    /// GPL (1.0, 2.0, 3.0)
    GPL(String),
    /// LGPL (2.0, 2.1, 3.0)
    LGPL(String),
    /// AGPL (1.0, 3.0)
    AGPL(String),
    /// Mozilla Public License (1.0, 1.1, 2.0)
    MPL(String),
    /// BSD (2-clause, 3-clause, 4-clause)
    BSD(String),
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
        // TODO(geekmasher): This could be improved
        match value.to_lowercase().as_str() {
            value if value.contains("apache") => {
                // apache-1.0 or apache-2.0
                if let Some((_, version)) = value.split_once('-') {
                    License::Apache(String::from(version.trim()))
                } else {
                    License::Apache(String::from(value))
                }
            }
            value if value.contains("mit") => License::MIT,
            value if value.starts_with("gpl") => {
                if let Some((_, version)) = value.split_once('-') {
                    License::GPL(String::from(version.trim()))
                } else {
                    License::GPL(String::from(value))
                }
            }
            value if value.starts_with("lgpl") => {
                if let Some((_, version)) = value.split_once('-') {
                    License::LGPL(String::from(version.trim()))
                } else {
                    License::LGPL(String::from(value))
                }
            }
            value if value.starts_with("agpl") => {
                if let Some((_, version)) = value.split_once('-') {
                    License::AGPL(String::from(version.trim()))
                } else {
                    License::AGPL(String::from(value))
                }
            }
            value if value.starts_with("mpl") => {
                if let Some((_, version)) = value.split_once('-') {
                    License::MPL(String::from(version.trim()))
                } else {
                    License::MPL(String::from(value))
                }
            }
            value if value.starts_with("bsd") => {
                if let Some((_, version)) = value.split_once('-') {
                    License::BSD(String::from(version.trim()))
                } else {
                    License::BSD(String::from(value))
                }
            }
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
        let license = License::from("Apache-2.0");
        assert_eq!(license, License::Apache(String::from("2.0")));
    }

    #[test]
    fn test_license_versions() {
        let license = License::from("GPL-3.0");
        assert_eq!(license, License::GPL(String::from("3.0")));

        let license = License::from("AGPL-3.0");
        assert_eq!(license, License::AGPL(String::from("3.0")));

        let license = License::from("MPL-3.0");
        assert_eq!(license, License::MPL(String::from("3.0")));
    }
}
