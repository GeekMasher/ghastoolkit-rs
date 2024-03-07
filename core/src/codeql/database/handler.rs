use std::path::PathBuf;

use crate::{codeql::CodeQLLanguage, CodeQL, CodeQLDatabase, GHASError};

/// CodeQL Database Handler
#[derive(Debug, Clone)]
pub struct CodeQLDatabaseHandler<'db, 'ql> {
    path: PathBuf,
    database: &'db CodeQLDatabase,
    codeql: &'ql CodeQL,

    command: Option<String>,
    overwrite: bool,
}

impl<'db, 'ql> CodeQLDatabaseHandler<'db, 'ql> {
    /// Create a new CodeQL Database Handler
    pub fn new(database: &'db CodeQLDatabase, codeql: &'ql CodeQL) -> Self {
        Self {
            path: PathBuf::new(),
            database,
            codeql,
            command: None,
            overwrite: false,
        }
    }

    /// Set the path of the database (defaults to the path set in the database)
    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = path;
        self
    }

    /// Set the build command to create the database (for compiled languages)
    pub fn command(mut self, command: String) -> Self {
        self.command = Some(command);
        self
    }

    /// Overwrite the database if it exists
    pub fn overwrite(mut self) -> Self {
        self.overwrite = true;
        self
    }

    /// Create a new CodeQL Database using the provided database
    pub fn create(&self) -> Result<(), GHASError> {
        let args = self.create_cmd()?;

        // Create path
        if !self.path.exists() {
            std::fs::create_dir_all(&self.path)?;
        }

        self.codeql.run(args)?;
        Ok(())
    }

    /// Create the command to create the database
    fn create_cmd(&self) -> Result<Vec<&str>, GHASError> {
        let mut args = vec!["database", "create"];

        // Check if language is set
        if self.database.language != CodeQLLanguage::None {
            args.extend(vec!["-l", &self.database.language()]);
        } else {
            return Err(GHASError::CodeQLDatabaseError(
                "No language provided".to_string(),
            ));
        }
        // Add source root
        if let Some(source) = &self.database.source {
            args.extend(vec!["-s", source.to_str().expect("Invalid Source Root")]);
        } else {
            return Err(GHASError::CodeQLDatabaseError(
                "No source root provided".to_string(),
            ));
        }
        // Overwrite the database if it exists
        if self.overwrite {
            args.push("--overwrite");
        }

        // Add the path to the database
        let path = self.database.path.to_str().expect("Invalid Database Path");
        args.push(path);

        Ok(args)
    }
}
