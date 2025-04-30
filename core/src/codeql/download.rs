//! # CodeQL CLI Download

use super::CodeQL;
use crate::GHASError;
use ghactions::{ToolCache, ToolPlatform};

impl CodeQL {
    /// Download the CodeQL CLI from GitHub using the GitHub Actions Tool Cache
    pub async fn download(client: &octocrab::Octocrab) -> Result<Self, GHASError> {
        let mut codeql = CodeQL::default();
        codeql.download_version(client, "latest").await?;
        Ok(codeql)
    }

    /// Check and install the CodeQL CLI if not already installed
    pub async fn install(
        &mut self,
        client: &octocrab::Octocrab,
        version: &str,
    ) -> Result<(), GHASError> {
        if !self.is_installed().await {
            self.download_version(client, version).await?;
        }
        Ok(())
    }

    /// Download the latest version of the CodeQL CLI from GitHub
    pub async fn download_latest(&mut self, client: &octocrab::Octocrab) -> Result<(), GHASError> {
        self.download_version(client, "latest").await
    }

    /// Download the CodeQL CLI from GitHub using the GitHub Actions Tool Cache
    pub async fn download_version(
        &mut self,
        client: &octocrab::Octocrab,
        version: &str,
    ) -> Result<(), GHASError> {
        let toolcache = ToolCache::new();
        let path = toolcache.new_tool_path("codeql", version);

        let codeql_archive = path.join("codeql.zip");
        log::debug!("CodeQL CLI archive path: {:?}", codeql_archive);
        log::debug!("CodeQL CLI directory path: {:?}", path);

        // CodeQL CLI names for the different platforms
        let platform = CodeQL::codeql_platform_str(&toolcache)?;
        log::debug!("CodeQL CLI platform: {}", platform);

        if !codeql_archive.exists() {
            let release = CodeQL::get_codeql_release(client, version).await?;
            log::debug!("CodeQL CLI version {} found on GitHub", release.tag_name);

            let codeql_str = format!("{}.zip", platform);
            log::debug!("CodeQL CLI asset name: {}", codeql_str);
            let Some(asset) = release
                .assets
                .iter()
                .find(|a| a.name == codeql_str.as_str())
            else {
                return Err(GHASError::CodeQLError(
                    "CodeQL CLI asset not found".to_string(),
                ));
            };

            if let Some(parent) = codeql_archive.parent() {
                log::debug!("Creating parent directory: {:?}", parent);
                std::fs::create_dir_all(parent)?;
            }

            log::info!(
                "Downloading CodeQL CLI from GitHub: {}",
                asset.browser_download_url
            );
            toolcache.download_asset(&asset, &codeql_archive).await?;
        }

        log::info!("Extracting asset to {:?}", path);
        toolcache.extract_archive(&codeql_archive, &path).await?;

        let codeql_dir = path.join("codeql");
        if !codeql_dir.exists() {
            return Err(GHASError::CodeQLError(
                "CodeQL CLI directory not found".to_string(),
            ));
        }
        log::info!("CodeQL CLI extracted to {:?}", codeql_dir);

        self.set_path(codeql_dir.join("codeql"));
        Ok(())
    }

    async fn get_codeql_release(
        client: &octocrab::Octocrab,
        version: &str,
    ) -> Result<octocrab::models::repos::Release, GHASError> {
        if version == "latest" {
            log::debug!("Fetching latest CodeQL CLI release");
            Ok(client
                .repos("github", "codeql-cli-binaries")
                .releases()
                .get_latest()
                .await?)
        } else {
            log::debug!("Fetching CodeQL CLI release by tag: {}", version);
            Ok(client
                .repos("github", "codeql-cli-binaries")
                .releases()
                .get_by_tag(version)
                .await?)
        }
    }

    /// Convert the toolcache platform to a string for the CodeQL CLI
    fn codeql_platform_str(toolcache: &ToolCache) -> Result<&str, GHASError> {
        Ok(match toolcache.platform() {
            ToolPlatform::Linux => "codeql-linux64",
            ToolPlatform::MacOS => "codeql-osx64",
            ToolPlatform::Windows => "codeql-win64",
            _ => {
                return Err(GHASError::CodeQLError(
                    "Unsupported platform for CodeQL CLI".to_string(),
                ));
            }
        })
    }
}
