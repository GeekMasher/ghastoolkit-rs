use anyhow::Result;
use ghastoolkit::{CodeQL, CodeQLDatabase, CodeQLDatabases, Repository, codeql::CodeQLLanguage};
use log::info;
use std::env::temp_dir;

mod cli;
mod codeql;
mod codescanning;
mod prompts;
mod secretscanning;

use cli::OutputFormat;
use codeql::{download_databases, output_databases_json};
use codescanning::code_scanning;
use prompts::{prompt_language, prompt_text};
use secretscanning::secret_scanning;

use self::codeql::list_languages;

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = cli::init();

    let github = arguments.github();
    let mut repository: Repository = match arguments.repository() {
        Ok(repo) => repo,
        Err(_) => Repository::try_from(
            prompt_text("GitHub Repository:")
                .expect("Failed to get repository")
                .as_str(),
        )
        .expect("Failed to parse repository"),
    };

    log::debug!("GitHub: {}", github);

    match arguments.commands {
        Some(cli::ArgumentCommands::Secretscanning { .. }) => {
            let args = arguments.commands.expect("Args issue");
            secret_scanning(&github, &repository, &args).await
        }
        Some(cli::ArgumentCommands::Codescanning { audit }) => {
            code_scanning(&github, &repository, audit).await
        }
        Some(cli::ArgumentCommands::Codeql {
            codeql_path,
            codeql_databases,
            list,
            repo,
            languages,
            language,
            suite,
            download,
            threads,
            ram,
        }) => {
            // Setup CodeQL
            let codeql = if let Some(codeql_path) = codeql_path {
                CodeQL::init()
                    .path(codeql_path)
                    .threads(threads.unwrap_or_default())
                    .ram(ram.unwrap_or_default())
                    .build()
                    .await?
            } else {
                CodeQL::init()
                    .threads(threads.unwrap_or_default())
                    .ram(ram.unwrap_or_default())
                    .build()
                    .await?
            };
            log::debug!("CodeQL :: {:?}", codeql);

            let mut databases = CodeQLDatabases::new();
            // This does not load the databases, it just sets the path
            databases.set_path(&codeql_databases);

            if list {
                databases = CodeQLDatabases::load(codeql_databases);

                match arguments.format {
                    OutputFormat::Json => {
                        output_databases_json(&databases, &arguments.output).await?;
                    }
                    _ => {
                        log::info!("Databases :: {}", databases.len());

                        for database in databases {
                            log::info!("{}", database);
                        }
                    }
                }
                return Ok(());
            } else if languages {
                list_languages(&codeql).await?;
            } else if download {
                download_databases(&mut databases, &repository, &github, language).await?;
            } else if repo {
                log::info!("Repository Mode :: {}", repository);

                let mut tempdir = temp_dir();
                tempdir.push("codeql-code");
                tempdir.push(repository.name());

                if tempdir.exists() {
                    tokio::fs::remove_dir_all(&tempdir).await?;
                }

                log::info!("Cloning repository to :: {}", tempdir.display());
                let _ = github.clone_repository(&mut repository, &tempdir.display().to_string());

                let language: CodeQLLanguage = match language {
                    Some(language) => CodeQLLanguage::from(language),
                    None => {
                        let languages = codeql.get_languages().await?;
                        prompt_language("Select Language: ", &languages)
                            .expect("Failed to select language")
                    }
                };

                let mut database = CodeQLDatabase::init()
                    .source(tempdir)
                    .language(language.clone())
                    .repository(&repository)
                    .build()?;

                if !database.path().exists() {
                    tokio::fs::create_dir_all(database.path()).await?;
                }

                log::info!("Database :: {}", database);
                log::info!("Creating database :: {}", database.path().display());

                codeql.database(&database).overwrite().create().await?;

                // Reload the database after creation
                database.reload()?;

                let sarif = database.path().join("results.sarif");
                log::info!("Results :: {:?}", &sarif);

                log::info!("Analyzing database :: {}", database);
                codeql
                    .database(&database)
                    .queries(suite.unwrap_or("default".to_string()))
                    .analyze()
                    .await?;

                let results = codeql.sarif(sarif)?;

                log::info!("Results :: {:?}", results.get_results().len());
                for result in results.get_results() {
                    info!("{}", result);
                }
            }

            log::info!("Completed!");
            Ok(())
        }
        None => {
            // Default mode
            Ok(())
        }
    }
}
