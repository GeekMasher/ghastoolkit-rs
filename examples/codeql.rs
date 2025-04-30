use anyhow::Result;
use ghastoolkit::{CodeQL, GitHub, Repository};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let github = GitHub::default();
    let repo = Repository::parse("GeekMasher/ghastoolkit-rs").expect("Failed to parse repository");
    println!("Repository :: {:#?}", repo);

    let mut codeql = CodeQL::init().build().await?;

    if !codeql.is_installed().await {
        println!("CodeQL CLI is not installed, installing latest version...");
        codeql.install(&github.octocrab(), "latest").await?;

        println!("CodeQL CLI downloaded successfully");
    }

    Ok(())
}
