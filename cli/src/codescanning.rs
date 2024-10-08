use anyhow::Result;
use ghastoolkit::{GitHub, Repository};

pub async fn code_scanning(github: &GitHub, repository: &Repository, audit: bool) -> Result<()> {
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
