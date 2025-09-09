//! CodeQL CLI Packs
use super::{CodeQL, models::ResolvedPacks};
use crate::{CodeQLPack, CodeQLPacks};
use anyhow::Result;

impl CodeQL {
    /// Resolve CodeQL Packs using the CodeQL CLI
    pub async fn resolve_packs(&self) -> Result<CodeQLPacks> {
        let packs = self.resolve_pack_command().await?;

        let mut codeql_packs = CodeQLPacks::default();

        for step in packs.steps {
            // We only care about packs resolved by name and version
            if step.r#type != "by-name-and-version" {
                continue;
            }

            for (_pack_name, pack_version_info) in step.found {
                for (_version, info) in pack_version_info {
                    let pack = CodeQLPack::load(info.path)?;
                    // TODO: Handle version properly
                    codeql_packs.add(pack);
                }
            }
        }

        Ok(codeql_packs)
    }

    /// Run the `codeql resolve packs --format json` command
    async fn resolve_pack_command(&self) -> Result<ResolvedPacks> {
        let mut cmd = vec![
            "resolve".to_string(),
            "packs".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ];

        if !self.search_path.is_empty() {
            cmd.push("--search-path".to_string());
            cmd.push(self.search_paths());
        }

        let output = self.run(cmd).await?;
        let packs: ResolvedPacks = serde_json::from_str(&output)?;
        Ok(packs)
    }
}
