use std::path::PathBuf;

use crate::{
    codeql::{database::queries::CodeQLQueries, CodeQLLanguage},
    utils::sarif::Sarif,
    CodeQL, CodeQLDatabase, CodeQLDatabases, GHASError,
};

/// CodeQL Database Handler
#[derive(Debug, Clone)]
pub struct CodeQLDatabaseHandler<'db, 'ql> {
    /// Reference to the CodeQL Database
    database: &'db CodeQLDatabase,
    /// Reference to the CodeQL instance
    codeql: &'ql CodeQL,
    /// Query / Pack / Suites
    queries: CodeQLQueries,
    /// Build command to create the database (for compiled languages)
    command: Option<String>,
    /// Output for Analysis
    output: PathBuf,
    /// Format for Analysis
    output_format: String,
    /// Overwrite the database if it exists
    overwrite: bool,
}

impl<'db, 'ql> CodeQLDatabaseHandler<'db, 'ql> {
    /// Create a new CodeQL Database Handler
    pub fn new(database: &'db CodeQLDatabase, codeql: &'ql CodeQL) -> Self {
        Self {
            database,
            codeql,
            // Default to standard query packs
            queries: CodeQLQueries::language_default(database.language.language()),
            command: None,
            output: CodeQLDatabaseHandler::default_results(database),
            output_format: String::from("sarif-latest"),
            overwrite: false,
        }
    }

    /// Set the build command to create the database (for compiled languages)
    pub fn command(mut self, command: String) -> Self {
        self.command = Some(command);
        self
    }

    /// Set the output for Analysis
    pub fn output(mut self, output: PathBuf) -> Self {
        self.output = output.clone();
        self
    }

    /// Set the queries / packs / suites to use for the analysis
    pub fn queries(mut self, queries: CodeQLQueries) -> Self {
        self.queries = queries;
        self
    }

    /// Overwrite the database if it exists
    pub fn overwrite(mut self) -> Self {
        self.overwrite = true;
        self
    }

    /// Create a new CodeQL Database using the provided database
    pub fn create(&mut self) -> Result<(), GHASError> {
        let args = self.create_cmd()?;

        // Create path
        if !self.database.path().exists() {
            std::fs::create_dir_all(self.database.path())?;
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

    pub(crate) fn default_results(database: &CodeQLDatabase) -> PathBuf {
        let mut path = CodeQLDatabases::default_results();

        if let Some(ref repo) = database.repository {
            path.push(format!(
                "{}-{}-{}.sarif",
                database.language(),
                repo.owner(),
                repo.name(),
            ));
        } else if database.language != CodeQLLanguage::None {
            path.push(format!(
                "{}-{}.sarif",
                database.language.language(),
                database.name
            ));
        } else {
            path.push(format!("{}.sarif", database.name.clone()));
        }

        path
    }
    /// Analyze the database
    pub fn analyze(&self) -> Result<Sarif, GHASError> {
        let args = self.analyze_cmd()?;

        self.codeql.run(args)?;
        Sarif::try_from(self.output.clone())
    }

    pub(crate) fn analyze_cmd(&self) -> Result<Vec<&str>, GHASError> {
        let mut args = vec!["database", "analyze"];

        // Output and Format
        if let Some(path) = &self.output.to_str() {
            args.extend(vec!["--output", path]);
        } else {
            return Err(GHASError::CodeQLDatabaseError(
                "No output path provided".to_string(),
            ));
        }
        args.extend(vec!["--format", self.output_format.as_str()]);

        // Add the path to the database
        let path = self.database.path.to_str().expect("Invalid Database Path");
        args.push(path);

        Ok(args)
    }
}
