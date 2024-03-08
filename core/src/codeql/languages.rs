use std::fmt::Display;

/// Languages supported by CodeQL.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CodeQLLanguage {
    /// C Programming Language
    C,
    /// C++ Programming Language
    Cpp,
    /// C# Programming Language
    CSharp,
    /// Go / GoLang Programming Language
    Go,
    /// Java Programming Language
    Java,
    /// JavaScript Programming Language
    JavaScript,
    /// Kotlin Programming Language
    Kotlin,
    /// Python Programming Language
    Python,
    /// TypeScript Programming Language
    TypeScript,
    /// Swift Programming Language
    Swift,
    /// Ruby Programming Language
    Ruby,
    /// Secondary languages (properties, csv, yaml, html, etc)
    Secondary(String),
    /// Custom Language
    Custom(String),
    /// No language
    #[default]
    None,
}

impl CodeQLLanguage {
    /// Get the pretty name of the language
    pub fn pretty(&self) -> &str {
        match self {
            CodeQLLanguage::C | CodeQLLanguage::Cpp => "C / C++",
            CodeQLLanguage::CSharp => "C#",
            CodeQLLanguage::Go => "Go",
            CodeQLLanguage::Java | CodeQLLanguage::Kotlin => "Java / Kotlin",
            CodeQLLanguage::JavaScript | CodeQLLanguage::TypeScript => "JavaScript / TypeScript",
            CodeQLLanguage::Python => "Python",
            CodeQLLanguage::Swift => "Swift",
            CodeQLLanguage::Ruby => "Ruby",
            CodeQLLanguage::Secondary(a) => match a.as_str() {
                "properties" => "Properties",
                "csv" => "CSV",
                "yaml" => "YAML",
                "xml" => "XML",
                "html" => "HTML",
                _ => a,
            },
            CodeQLLanguage::Custom(a) => a,
            CodeQLLanguage::None => "None",
        }
    }

    /// Get the language string for CodeQL (aliases are supported)
    pub fn language(&self) -> &str {
        match self {
            CodeQLLanguage::C | CodeQLLanguage::Cpp => "cpp",
            CodeQLLanguage::CSharp => "csharp",
            CodeQLLanguage::Go => "go",
            CodeQLLanguage::Java | CodeQLLanguage::Kotlin => "java",
            CodeQLLanguage::JavaScript | CodeQLLanguage::TypeScript => "javascript",
            CodeQLLanguage::Python => "python",
            CodeQLLanguage::Swift => "swift",
            CodeQLLanguage::Ruby => "ruby",
            CodeQLLanguage::Secondary(a) => a,
            CodeQLLanguage::Custom(a) => a,
            CodeQLLanguage::None => "none",
        }
    }

    /// Check if the language is a secondary language
    pub fn is_secondary(&self) -> bool {
        matches!(self, CodeQLLanguage::Secondary(_))
    }

    /// Check if the language is None
    pub fn is_none(&self) -> bool {
        matches!(self, CodeQLLanguage::None)
    }

    /// Check if the language is custom
    pub fn is_custom(&self) -> bool {
        matches!(self, CodeQLLanguage::Custom(_))
    }

    /// Get the list of supported languages
    pub fn list() -> Vec<&'static str> {
        // TODO(geekmasher): This could be a lot cleaner
        vec![
            "c",
            "cpp",
            "csharp",
            "go",
            "java",
            "javascript",
            "kotlin",
            "python",
            "typescript",
            "swift",
            "ruby",
        ]
    }
}

impl Display for CodeQLLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty())
    }
}

impl From<&str> for CodeQLLanguage {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "c" => CodeQLLanguage::C,
            "cpp" | "c++" => CodeQLLanguage::Cpp,
            "csharp" | "c#" => CodeQLLanguage::CSharp,
            "go" | "golang" => CodeQLLanguage::Go,
            "java" => CodeQLLanguage::Java,
            "kotlin" => CodeQLLanguage::Kotlin,
            "javascript" | "js" => CodeQLLanguage::JavaScript,
            "typescript" | "ts" => CodeQLLanguage::TypeScript,
            "python" | "py" => CodeQLLanguage::Python,
            "swift" => CodeQLLanguage::Swift,
            "ruby" => CodeQLLanguage::Ruby,
            "properties" | "csv" | "yaml" | "xml" | "html" => {
                CodeQLLanguage::Secondary(s.to_string())
            }
            _ => CodeQLLanguage::None,
        }
    }
}

impl From<String> for CodeQLLanguage {
    fn from(s: String) -> Self {
        CodeQLLanguage::from(s.as_str())
    }
}

impl From<Option<String>> for CodeQLLanguage {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => CodeQLLanguage::from(s),
            None => CodeQLLanguage::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codeql::CodeQLLanguage;

    #[test]
    fn test_parsing() {
        let lang1 = CodeQLLanguage::from("c");
        assert_eq!(lang1, CodeQLLanguage::C);

        let lang2 = CodeQLLanguage::from("cpp");
        assert_eq!(lang2, CodeQLLanguage::Cpp);

        let lang3 = CodeQLLanguage::from("csharp");
        assert_eq!(lang3, CodeQLLanguage::CSharp);

        let lang4 = CodeQLLanguage::from("kotlin");
        assert_eq!(lang4, CodeQLLanguage::Kotlin);
        assert_eq!(lang4.language(), "java");
    }

    #[test]
    fn test_pretty() {
        let py = CodeQLLanguage::Python;
        assert_eq!(py.pretty(), "Python");
        assert_eq!(py.language(), "python");

        let cs = CodeQLLanguage::CSharp;
        assert_eq!(cs.pretty(), "C#");
        assert_eq!(cs.language(), "csharp");
    }

    #[test]
    fn test_incorrect() {
        // RIP Rust
        let lang = CodeQLLanguage::from("rust");
        assert_eq!(lang, CodeQLLanguage::None);

        let lang = CodeQLLanguage::from(Some("rust".to_string()));
        assert_eq!(lang, CodeQLLanguage::None);

        let lang = CodeQLLanguage::from(None);
        assert_eq!(lang, CodeQLLanguage::None);
    }
}
