use std::env::temp_dir;

use anyhow::Result;
use ghastoolkit::{
    codeql::{database::queries::CodeQLQueries, CodeQLLanguage},
    CodeQL, CodeQLDatabase, CodeQLDatabases, GitHub, Repository,
};

mod cli;
mod prompts;

use log::{debug, info};

use crate::prompts::{prompt_select, prompt_text};

async fn code_scanning(github: &GitHub, repository: &Repository, audit: bool) -> Result<()> {
    println!("\n ----- Code Scanning -----");

    if github.code_scanning(repository).is_enabled().await {
        let analyses = github
            .code_scanning(repository)
            .analyses()
            .tool_name("codeql")
            .per_page(1)
            .send()
            .await?;

        for analysis in analyses {
            println!(
                "Code Scanning Analysis :: {} ({})",
                analysis.tool.name, analysis.tool.version
            );
        }

        if audit {
            let alerts = github
                .code_scanning(repository)
                .list()
                .state("open")
                .send()
                .await?;

            for alert in alerts {
                println!(
                    "Code Scanning Alert :: {} - {} - {}",
                    alert.tool.name, alert.rule.name, alert.rule.severity
                );
            }
        }
    } else {
        return Err(anyhow::anyhow!(
            "Code Scanning is not enabled for this repository"
        ));
    }

    Ok(())
}

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
        Some(cli::ArgumentCommands::Codescanning { audit }) => {
            code_scanning(&github, &repository, audit).await
        }
        Some(cli::ArgumentCommands::Codeql {
            codeql_path,
            codeql_databases,
            list,
            repo,
            language,
            threads,
            ram,
        }) => {
            // Setup CodeQL
            let codeql = CodeQL::init()
                .path(codeql_path.unwrap_or_default())
                .threads(threads.unwrap_or_default())
                .ram(ram.unwrap_or_default())
                .build()?;

            info!("CodeQL :: {}", codeql);
            info!(
                "CodeQL Languages Loaded :: {}",
                codeql.get_languages().await?.len()
            );

            if list {
                let databases = CodeQLDatabases::from(codeql_databases);
                info!("Databases :: {}", databases.len());
                for database in databases {
                    info!("{}", database);
                }
                return Ok(());
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

                let language: CodeQLLanguage = CodeQLLanguage::from(match language {
                    Some(language) => language,
                    None => {
                        prompt_select("Select Language: ", &CodeQLLanguage::list())?.to_string()
                    }
                });

                let mut database = CodeQLDatabase::init()
                    .source(tempdir.display().to_string())
                    .language(language.to_string())
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

                let queries = CodeQLQueries::language_default(language.language());

                info!("Analyzing database :: {}", database);
                let results = codeql
                    .database(&database)
                    .queries(queries)
                    .analyze()
                    .await?;

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
