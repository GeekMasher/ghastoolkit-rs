use anyhow::Result;
use ghastoolkit::{CodeQL, CodeQLDatabase, CodeQLDatabases, Repository, codeql::CodeQLLanguage};
use log::{debug, info};
use secretscanning::secret_scanning;
use std::env::temp_dir;

mod cli;
mod codescanning;
mod prompts;
mod secretscanning;

use crate::prompts::{prompt_languages, prompt_text};
use codescanning::code_scanning;

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

    debug!("GitHub :: {}", github);
    debug!("Repository :: {}", repository);

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
            let codeql = CodeQL::init()
                .path(codeql_path.unwrap_or_default())
                .threads(threads.unwrap_or_default())
                .ram(ram.unwrap_or_default())
                .build()
                .await?;
            info!("CodeQL :: {}", codeql);

            if list {
                let databases = CodeQLDatabases::from(codeql_databases);
                info!("Databases :: {}", databases.len());
                for database in databases {
                    info!("{}", database);
                }
                return Ok(());
            } else if languages {
                let languages = codeql.get_languages().await?;
                info!("CodeQL Languages Loaded :: {}", languages.len());

                for language in languages {
                    info!("> {}", language);
                }
            } else if download {
                let cs_languages = github
                    .code_scanning(&repository)
                    .list_codeql_databases()
                    .await?;
                info!("CodeQL Languages Loaded :: {}", cs_languages.len());

                let available_languages = cs_languages
                    .iter()
                    .map(|l| CodeQLLanguage::from(l.language.to_string()))
                    .collect::<Vec<CodeQLLanguage>>();

                let select_languages =
                    prompt_languages("Select the language to download:", &available_languages)?;
                log::info!("Selected languages: {:?}", select_languages);

                let dbpath = github
                    .code_scanning(&repository)
                    .download_codeql_database(select_languages, &codeql_databases)
                    .await?;
                let db = CodeQLDatabase::load(&dbpath)?;

                log::info!("Downloaded CodeQL databases to {}", db.path().display());
            } else if repo {
                info!("Repository Mode :: {}", repository);

                let mut tempdir = temp_dir();
                tempdir.push("codeql-code");
                tempdir.push(repository.name());

                if tempdir.exists() {
                    std::fs::remove_dir_all(&tempdir)?;
                }

                info!("Cloning repository to :: {}", tempdir.display());
                let _ = github.clone_repository(&mut repository, &tempdir.display().to_string());

                let language: CodeQLLanguage = match language {
                    Some(language) => CodeQLLanguage::from(language),
                    None => {
                        let languages = codeql.get_languages().await?;
                        prompt_languages("Select Language: ", &languages)
                            .expect("Failed to select language")
                    }
                };

                let mut database = CodeQLDatabase::init()
                    .source(tempdir)
                    .language(language.clone())
                    .repository(&repository)
                    .build()?;

                if !database.path().exists() {
                    std::fs::create_dir_all(database.path())?;
                }

                info!("Database :: {}", database);
                info!("Creating database :: {}", database.path().display());

                codeql.database(&database).overwrite().create().await?;

                // Reload the database after creation
                database.reload()?;

                let sarif = database.path().join("results.sarif");
                info!("Results :: {:?}", &sarif);

                info!("Analyzing database :: {}", database);
                codeql
                    .database(&database)
                    .queries(suite.unwrap_or("default".to_string()))
                    .analyze()
                    .await?;

                let results = codeql.sarif(sarif)?;

                info!("Results :: {:?}", results.get_results().len());
                for result in results.get_results() {
                    info!("{}", result);
                }
            }

            info!("Completed!");
            Ok(())
        }
        None => {
            // Default mode
            Ok(())
        }
    }
}
