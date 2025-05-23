use anyhow::{Context, Result};
use ghastoolkit::codeql::CodeQLLanguage;
use ghastoolkit::{CodeQL, CodeQLDatabases, GitHub, Repository};
use std::path::PathBuf;

use crate::prompts::prompt_languages;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeQLDatabaseJsonItem {
    pub name: String,
    pub language: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn output_databases_json(
    databases: &CodeQLDatabases,
    output: &Option<PathBuf>,
) -> anyhow::Result<()> {
    let mut data: Vec<CodeQLDatabaseJsonItem> = Vec::new();

    for database in databases.databases() {
        data.push(CodeQLDatabaseJsonItem {
            name: database.name().to_string(),
            language: database.language().to_string(),
            path: database.path().display().to_string(),
            created_at: database.created_at(),
        });
    }
    let json = serde_json::to_string_pretty(&data)?;

    if let Some(output) = output {
        tokio::fs::write(output, json).await?;
    } else {
        println!("{}", json);
    }
    Ok(())
}

pub async fn list_languages(codeql: &CodeQL) -> Result<()> {
    let languages = codeql.get_languages().await?;
    log::info!("CodeQL Languages Loaded :: {}", languages.len());

    for language in languages {
        log::info!("> {}", language);
    }

    Ok(())
}

pub async fn download_databases(
    databases: &mut CodeQLDatabases,
    repository: &Repository,
    github: &GitHub,
    language: Option<String>,
) -> anyhow::Result<()> {
    log::info!("Download CodeQL Database");
    log::info!("Repository  :: {}", repository);

    let db_languages = if let Some(language) = language {
        vec![CodeQLLanguage::from(language)]
    } else {
        let cs_languages = github
            .code_scanning(&repository)
            .list_codeql_databases()
            .await
            .context("Failed to list CodeQL databases")?;
        log::info!("CodeQL Languages Loaded :: {}", cs_languages.len());

        let available_languages = cs_languages
            .iter()
            .map(|l| CodeQLLanguage::from(l.language.to_string()))
            .collect::<Vec<CodeQLLanguage>>();

        prompt_languages("Select the language to download:", &available_languages)
            .context("Failed to select language")?
    };
    log::info!("Selected languages: {:?}", db_languages);

    for lang in db_languages {
        log::debug!("{:?}", lang);

        let db = databases
            .download_language(&repository, &github, lang)
            .await?;
        log::info!("Downloaded CodeQL Database: {}", db.name());
    }
    log::info!("CodeQL databases downloaded successfully");

    log::info!("CodeQL Databases Loaded :: {}", databases.len());
    for db in databases.databases() {
        log::info!("Database Name       : {}", db.name());
        log::info!("Database Path       : {}", db.path().display());
        log::info!("Database Language   : {}", db.language());
        if let Some(created_at) = db.created_at() {
            log::info!("Database Created At : {}", created_at);
        }
    }

    Ok(())
}
