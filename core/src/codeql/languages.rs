//! # CodeQL Languages
use std::fmt::{Debug, Display};
use std::path::PathBuf;

use super::CodeQLExtractor;

/// CodeQL Languages mappings
pub const CODEQL_LANGUAGES: [(&str, &str); 16] = [
    ("actions", "GitHub Actions"),
    ("c", "C/C++"),
    ("cpp", "C/C++"),
    ("c-cpp", "C/C++"),
    ("csharp", "C#"),
    ("java", "Java/Kotlin"),
    ("kotlin", "Java/Kotlin"),
    ("java-kotlin", "Java/Kotlin"),
    ("javascript", "Javascript/Typescript"),
    ("typescript", "Javascript/Typescript"),
    ("javascript-typescript", "Javascript/Typescript"),
    ("python", "Python"),
    ("go", "Go"),
    ("rust", "Rust"),
    ("ruby", "Rudy"),
    ("swift", "Swift"),
];

/// CodeQL Languages
#[derive(Debug, Clone, Default)]
pub struct CodeQLLanguages {
    languages: Vec<CodeQLLanguage>,
}

impl CodeQLLanguages {
    /// Create a new instance of CodeQLLanguages
    pub fn new(languages: Vec<CodeQLLanguage>) -> Self {
        CodeQLLanguages { languages }
    }
    /// Check if a language is supported by CodeQL
    pub fn check(&self, language: impl Into<String>) -> bool {
        let language = language.into();
        for lang in &self.languages {
            if lang.extractor.languages().contains(&language) {
                return true;
            }
        }
        false
    }

    /// Get all languages supported by CodeQL
    pub fn get_all(&self) -> &Vec<CodeQLLanguage> {
        &self.languages
    }
    /// Get all primary languages supported by CodeQL
    pub fn get_languages(&self) -> Vec<CodeQLLanguage> {
        self.languages
            .iter()
            .filter(|l| !l.is_secondary())
            .cloned()
            .collect()
    }
    /// Get all secondary languages supported by CodeQL
    pub fn get_secondary(&self) -> Vec<CodeQLLanguage> {
        self.languages
            .iter()
            .filter(|l| l.is_secondary())
            .cloned()
            .collect()
    }
}

/// Languages supported by CodeQL.
#[derive(Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CodeQLLanguage {
    name: String,
    extractor: CodeQLExtractor,
}

impl CodeQLLanguage {
    /// Get the pretty name of the language
    pub fn pretty(&self) -> &str {
        if CODEQL_LANGUAGES.iter().any(|(lang, _)| lang == &self.name) {
            CODEQL_LANGUAGES
                .iter()
                .find(|(lang, _)| lang == &self.name)
                .unwrap()
                .1
        } else if !self.extractor.name.is_empty() {
            &self.extractor.name
        } else {
            &self.name
        }
    }

    /// Get the language string for CodeQL (aliases are supported)
    pub fn language(&self) -> &str {
        if !self.extractor.name.is_empty() {
            &self.extractor.name
        } else {
            &self.name
        }
    }

    /// Check if the language is a secondary language
    pub fn is_secondary(&self) -> bool {
        matches!(
            self.extractor.name.as_str(),
            "properties" | "csv" | "yaml" | "xml" | "html"
        )
    }
}

impl Display for CodeQLLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty())
    }
}

impl Debug for CodeQLLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_secondary() {
            write!(f, "Secondary('{}')", self.pretty())
        } else {
            write!(f, "Primary('{}')", self.pretty())
        }
    }
}

impl From<(String, PathBuf)> for CodeQLLanguage {
    fn from(value: (String, PathBuf)) -> Self {
        CodeQLLanguage {
            name: value.0.clone(),
            extractor: CodeQLExtractor::load_path(value.1.clone()).unwrap(),
        }
    }
}

impl From<String> for CodeQLLanguage {
    fn from(value: String) -> Self {
        CodeQLLanguage {
            name: value.clone(),
            extractor: CodeQLExtractor::default(),
        }
    }
}

impl From<&str> for CodeQLLanguage {
    fn from(value: &str) -> Self {
        CodeQLLanguage {
            name: value.to_string(),
            extractor: CodeQLExtractor::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_languages() {
        let language = CodeQLLanguage::from("cpp".to_string());
        assert_eq!(language.language(), "cpp");
        assert_eq!(language.pretty(), "C/C++");

        let language = CodeQLLanguage::from("actions");
        assert_eq!(language.language(), "actions");
        assert_eq!(language.pretty(), "GitHub Actions");
    }

    #[test]
    fn test_unsupported() {
        let language = CodeQLLanguage::from("iac");
        assert_eq!(language.language(), "iac");
        assert_eq!(language.pretty(), "iac");
    }
}
