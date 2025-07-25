//! CodeQL Packs module
use std::env::home_dir;
use std::path::PathBuf;

use anyhow::Result;

use crate::CodeQLPack;

/// CodeQL Packs
#[derive(Debug, Clone, Default)]
pub struct CodeQLPacks {
    packs: Vec<CodeQLPack>,
}

impl CodeQLPacks {
    /// Get the number of packs
    pub fn len(&self) -> usize {
        self.packs.len()
    }
    /// Sort the packs by type (Library, Queries, Models, Testing)
    pub fn sort(&mut self) {
        self.packs.sort_by(|a, b| a.pack_type().cmp(&b.pack_type()));
    }
    /// Get the packs
    pub fn packs(&self) -> &[CodeQLPack] {
        &self.packs
    }
    /// Merge two CodeQL Packs
    pub fn merge(&mut self, other: &mut Self) {
        self.packs.append(&mut other.packs);
    }

    /// Get the CodeQL Packages Path
    pub(crate) fn codeql_packages_path() -> PathBuf {
        let homedir = home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .canonicalize()
            .unwrap_or_else(|_| PathBuf::from("."));
        homedir.join(".codeql").join("packages")
    }

    /// Load CodeQL Packs from a directory. It will recursively search for `qlpack.yml` files.
    pub fn load(path: impl Into<PathBuf>) -> Result<Self> {
        let path: PathBuf = path.into();
        let mut packs = Vec::new();

        for entry in walkdir::WalkDir::new(&path) {
            let entry = entry?;

            // Skip any subdirectories named `.codeql`
            // TODO: Is this the best way to handle this?
            if entry.path().to_str().unwrap().contains(".codeql") {
                continue;
            }

            if entry.file_name() == "qlpack.yml" {
                let pack = CodeQLPack::new(entry.path().display().to_string());
                packs.push(pack);
            }
        }

        Ok(Self { packs })
    }
}

impl IntoIterator for CodeQLPacks {
    type Item = CodeQLPack;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.packs.into_iter()
    }
}

impl Extend<CodeQLPack> for CodeQLPacks {
    fn extend<T: IntoIterator<Item = CodeQLPack>>(&mut self, iter: T) {
        self.packs.extend(iter);
    }
}
