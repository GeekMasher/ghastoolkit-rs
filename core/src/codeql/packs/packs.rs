//! CodeQL Packs module
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
        self.packs.sort_by(|a, b| a.get_type().cmp(&b.get_type()));
    }

    /// Load CodeQL Packs from a directory. It will recursively search for `qlpack.yml` files.
    pub fn load(path: impl Into<PathBuf>) -> Result<Self> {
        let path: PathBuf = path.into();
        let mut packs = Vec::new();

        for entry in walkdir::WalkDir::new(&path) {
            let entry = entry?;
            if entry.file_name() == "qlpack.yml" {
                let pack = CodeQLPack::new(entry.path());
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
