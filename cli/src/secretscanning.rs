use anyhow::Result;
use ghastoolkit::{secretscanning::secretalerts::SecretScanningSort, GitHub, Repository};

use crate::cli::ArgumentCommands;

pub async fn secret_scanning(
    github: &GitHub,
    repository: &Repository,
    args: &ArgumentCommands,
) -> Result<()> {
    if let ArgumentCommands::Secretscanning {
        state,
        r#type,
        validity,
        links,
    } = args
    {
        let octocrab = github.octocrab();
        println!("\n ----- Secret Scanning -----\n");

        let mut handle = github
            .secret_scanning(repository)
            .list()
            .sort(SecretScanningSort::Created)
            .state(state.clone().unwrap_or_default())
            .secret_type(r#type.clone().unwrap_or_default())
            .validity(validity.clone().unwrap_or_default())
            .send()
            .await?;

        let mut alerts = handle.take_items();

        while let Ok(Some(mut page)) = octocrab.get_page(&handle.next).await {
            alerts.extend(page.take_items());
            handle = page;
        }

        for alert in &alerts {
            println!(
                "> {} :: {} ({}, {:?})",
                alert.number,
                alert.secret_type_display_name,
                alert.state,
                alert.validity.as_ref().unwrap_or_else(|| {
                    &ghastoolkit::secretscanning::secretalerts::SecretScanningAlertValidity::Unknown
                })
            );
            if *links {
                println!("  > {}", alert.html_url);
            }
        }

        println!("\n Total Alerts :: {}", alerts.len());
    }

    Ok(())
}
