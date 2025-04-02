use anyhow::Result;
use ghastoolkit::{CodeQLDatabase, GitHub, Repository};

#[tokio::main]
async fn main() -> Result<()> {
    let github = GitHub::default();
    let repo = Repository::parse("GeekMasher/ghastoolkit").expect("Failed to parse repository");
    println!("Repository :: {:#?}", repo);

    let databases = CodeQLDatabase::download("./".into(), &repo, &github).await?;
    println!("Databases :: {:#?}", databases);

    Ok(())
}
