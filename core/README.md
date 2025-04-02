# GHASToolkit

This is the GitHub Advanced Security (GHAS) Toolkit in Rust.
This toolkit is designed to help developers and security researchers to interact with the GitHub Advanced Security API.

## ✨ Features

- [Core GHAS Library][code-core]
  - [Documentation][docs]
  - GitHub Cloud and Enterprise Server support
  - API Support
    - [x] [Code Scanning][github-code-scanning]
    - [x] 👷 [Secret Scanning][github-secret-scanning]
    - [x] 👷 [Supply Chain][github-supplychain]
      - [ ] 👷 [Dependabot][github-dependabot] (Security Alerts)
      - [ ] 👷 [Dependency Graph][github-depgraph] (SCA / SBOMs)
      - [ ] 👷 [Security Advisories][github-advisories]
- [CLI Tool][code-cli]

## 🚀 Usage

### GitHub APIs

You can use the `GitHub` and `Repository` structs to interact with the GitHub API.

```rust no_run
use ghastoolkit::{GitHub, Repository};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let github = GitHub::default();
    println!("GitHub :: {}", github);

    let repository = Repository::parse("geekmasher/ghastoolkit-rs@main")
        .expect("Failed to parse repository");
    println!("Repository :: {}", repository);

    Ok(())
}
```

### CodeQL

You can use the `CodeQL` struct to interact with the CodeQL CLI.

```rust no_run
use ghastoolkit::{CodeQL, CodeQLDatabase, CodeQLDatabases};
use ghastoolkit::{GitHub, Repository};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let codeql = CodeQL::new().await;
    println!("CodeQL :: {}", codeql);

    let languages = codeql.get_languages().await?;
    println!("Languages :: {:#?}", languages);

    // Get all CodeQL databases from the default path
    let databases = CodeQLDatabases::default();
    for database in databases {
        println!("Database :: {}", database);
    }

    // Create a new CodeQL database
    let database = CodeQLDatabase::init()
        .name("my-project")
        .language("javascript")
        .path("/path/to/code".to_string())
        .build()
        .expect("Failed to create CodeQL database");

    // Create the database using the CodeQL CLI
    codeql.database(&database)
        .create()
        .await?;

    // Run a CodeQL query
    codeql.database(&database)
        .analyze()
        .await?;


    // You can also download a CodeQL Database from GitHub
    let github = GitHub::default();
    let repo = Repository::parse("geekmasher/ghastoolkit-rs@main")
        .expect("Failed to parse repository");

    let databases = CodeQLDatabase::download("./".into(), &repo, &github).await?;
    println!("Databases :: {:#?}", databases);

    Ok(())
}
```

