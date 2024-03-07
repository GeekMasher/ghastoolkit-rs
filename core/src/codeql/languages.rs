use std::fmt::Display;

/// Languages supported by CodeQL.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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
    /// No language
    #[default]
    None,
}

impl CodeQLLanguage {
    /// Get the pretty name of the language
    pub fn pretty(&self) -> &str {
        match self {
            CodeQLLanguage::C => "C",
            CodeQLLanguage::Cpp => "C++",
            CodeQLLanguage::CSharp => "C#",
            CodeQLLanguage::Go => "Go",
            CodeQLLanguage::Java => "Java",
            CodeQLLanguage::JavaScript => "JavaScript",
            CodeQLLanguage::Kotlin => "Kotlin",
            CodeQLLanguage::Python => "Python",
            CodeQLLanguage::TypeScript => "TypeScript",
            CodeQLLanguage::Swift => "Swift",
            CodeQLLanguage::Ruby => "Ruby",
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
            CodeQLLanguage::None => "none",
        }
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
        write!(f, "{}", self.language())
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
