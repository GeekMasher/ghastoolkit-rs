//! # CodeQL Database Handler
use std::path::PathBuf;

use crate::{
    CodeQL, CodeQLDatabase, CodeQLDatabases, GHASError, codeql::database::queries::CodeQLQueries,
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
    /// Threat Models
    threat_models: Vec<String>,
    /// Model Packs
    model_packs: Vec<String>,
    /// SARIF Category
    category: Option<String>,
    /// Build command to create the database (for compiled languages)
    command: Option<String>,
    /// Output for Analysis
    output: PathBuf,
    /// Format for Analysis
    output_format: String,
    /// Overwrite the database if it exists
    overwrite: bool,
    /// Summary of the analysis
    summary: bool,
}

impl<'db, 'ql> CodeQLDatabaseHandler<'db, 'ql> {
    /// Create a new CodeQL Database Handler
    pub fn new(database: &'db CodeQLDatabase, codeql: &'ql CodeQL) -> Self {
        Self {
            database,
            codeql,
            // Default to standard query packs
            queries: CodeQLQueries::language_default(database.language.language()),
            threat_models: Vec::new(),
            model_packs: Vec::new(),
            category: None,
            command: None,
            output: CodeQLDatabaseHandler::default_results(database),
            output_format: String::from("sarif-latest"),
            overwrite: false,
            summary: true,
        }
    }

    /// Set the build command to create the database (for compiled languages)
    pub fn command(mut self, command: String) -> Self {
        self.command = Some(command);
        log::trace!("Setting build command: {:?}", self.command);
        self
    }

    /// Set the output for Analysis
    pub fn output(mut self, output: PathBuf) -> Self {
        self.output = output.clone();
        log::trace!("Setting output path: {:?}", self.output);
        self
    }

    /// Set the CodeQL output path and format to `sarif` (JSON)
    pub fn sarif(mut self, output: impl Into<PathBuf>) -> Self {
        self.output = output.into();
        self.output_format = String::from("sarif-latest");
        log::trace!("Setting output format to SARIF");
        self
    }

    /// Set the CodeQL output path and format to `csv`
    pub fn csv(mut self, output: impl Into<PathBuf>) -> Self {
        self.output = output.into();
        self.output_format = String::from("csv");
        log::trace!("Setting output format to CSV");
        self
    }

    /// Set the queries / packs / suites to use for the analysis
    pub fn queries(mut self, queries: impl Into<CodeQLQueries>) -> Self {
        self.queries = queries.into();
        log::trace!("Setting queries: {:?}", self.queries);
        self
    }

    /// Set the query pack and suite to use for the analysis
    pub fn suite(mut self, queries: impl Into<String>) -> Self {
        let queries: String = queries.into();

        match queries.as_str() {
            "security-extended" => {
                self.queries = CodeQLQueries::language_default(self.database.language.language());
                self.queries.set_path(format!(
                    "codeql-suites/{}-security-extended.qls",
                    self.database.language.language()
                ));
            }
            "security-and-quality" => {
                self.queries = CodeQLQueries::language_default(self.database.language.language());
                self.queries.set_path(format!(
                    "codeql-suites/{}-security-and-quality.qls",
                    self.database.language.language()
                ));
            }
            "experimental" => {
                self.queries = CodeQLQueries::language_default(self.database.language.language());
                self.queries.set_path(format!(
                    "codeql-suites/{}-experimental.qls",
                    self.database.language.language()
                ));
            }
            "default" | "code-scanning" => {
                self.queries = CodeQLQueries::language_default(self.database.language.language());
                self.queries.set_path(format!(
                    "codeql-suites/{}-code-scanning.qls",
                    self.database.language.language()
                ));
            }
            _ => {
                self.queries = CodeQLQueries::from(queries.clone());
            }
        }

        log::trace!("Setting queries: {:?}", self.queries);
        self
    }

    /// Set a Threat Model for the analysis
    pub fn threat_model(mut self, threat_model: impl Into<String>) -> Self {
        self.threat_models.push(threat_model.into());
        log::trace!("Setting threat model: {:?}", self.threat_models);
        self
    }

    /// Set Threat Models for the analysis
    pub fn threat_models(mut self, threat_models: Vec<impl Into<String>>) -> Self {
        self.threat_models = threat_models.into_iter().map(|tm| tm.into()).collect();
        log::trace!("Setting threat models: {:?}", self.threat_models);
        self
    }

    /// Disable the default threat model
    pub fn disable_default_threat_model(mut self) -> Self {
        self.threat_models.push("!default".to_string());
        log::trace!("Disabling default threat model");
        self
    }

    /// Set Model Packs for the analysis
    pub fn model_pack(mut self, model_pack: impl Into<String>) -> Self {
        self.model_packs.push(model_pack.into());
        log::trace!("Setting model pack: {:?}", self.model_packs);
        self
    }

    /// Set Model Packs for the analysis
    ///
    /// This replaces any existing model packs with the provided ones
    pub fn model_packs(mut self, model_packs: Vec<impl Into<String>>) -> Self {
        self.model_packs = model_packs.into_iter().map(|p| p.into()).collect();
        log::trace!("Setting model packs: {:?}", self.model_packs);
        self
    }

    /// Set the SARIF category for the analysis
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        log::trace!("Setting SARIF category: {:?}", self.category);
        self
    }

    /// Overwrite the database if it exists
    pub fn overwrite(mut self) -> Self {
        self.overwrite = true;
        log::trace!("Overwriting database if it exists");
        self
    }

    /// Output the summary of the analysis
    pub fn summary(mut self, summary: bool) -> Self {
        self.summary = summary;
        log::trace!("Setting summary: {:?}", self.summary);
        self
    }

    /// Create a new CodeQL Database using the provided database
    pub async fn create(&mut self) -> Result<(), GHASError> {
        log::debug!("Creating CodeQL Database: {:?}", self.database);

        let args = self.create_cmd()?;

        // Create path
        if !self.database.path().exists() {
            log::debug!("Creating CodeQL Database Path: {:?}", self.database.path());
            std::fs::create_dir_all(self.database.path())?;
        }

        log::debug!("Creating CodeQL Database: {:?}", args);
        self.codeql.run(args).await?;
        Ok(())
    }

    fn create_cmd(&self) -> Result<Vec<String>, GHASError> {
        let mut args = vec!["database", "create"];

        // Check if language is set
        args.extend(vec!["-l", &self.database.language()]);

        // Add source root
        if let Some(source) = &self.database.source {
            args.extend(vec!["-s", source.to_str().expect("Invalid Source Root")]);
        } else {
            return Err(GHASError::CodeQLDatabaseError(
                "No source root provided".to_string(),
            ));
        }
        // Threat Models
        let tms = self.threat_models.join(",");
        if !tms.is_empty() {
            args.push("--threat-models");
            args.push(&tms);
        }
        // Model Packs
        let mps = self.model_packs.join(",");
        if !mps.is_empty() {
            args.push("--model-packs");
            args.push(&mps);
        }
        // Add Search Paths
        let search_paths = self.codeql.search_paths();
        if !search_paths.is_empty() {
            args.push("--search-path");
            args.push(&search_paths);
        }

        // Overwrite the database if it exists
        if self.overwrite {
            args.push("--overwrite");
        }
        if self.summary {
            args.push("--print-diagnostics-summary");
            args.push("--print-metrics-summary");
        }
        if let Some(category) = &self.category {
            args.push("--sarif-category");
            args.push(&category);
        }

        // Add the path to the database
        let path = self.database.path.to_str().expect("Invalid Database Path");
        args.push(path);

        Ok(args.iter().map(|s| s.to_string()).collect())
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
        } else {
            path.push(format!(
                "{}-{}.sarif",
                database.language.language(),
                database.name
            ));
        }

        path
    }

    /// Analyze the database
    pub async fn analyze(&self) -> Result<(), GHASError> {
        log::debug!("Analyzing CodeQL Database: {:?}", self.database);

        let args = self.analyze_cmd()?;

        log::debug!("Analyzing CodeQL Command :: {:?}", args);

        self.codeql.run(args).await?;

        log::debug!("CodeQL Database Analysis Complete");
        log::debug!("Output Path: {:?}", self.output);
        Ok(())
    }

    fn analyze_cmd(&self) -> Result<Vec<String>, GHASError> {
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

        // Search Paths
        let search_paths = self.codeql.search_paths();
        if !search_paths.is_empty() {
            args.push("--search-path");
            args.push(&search_paths);
        }

        // Add the path to the database
        let path = self.database.path.to_str().expect("Invalid Database Path");
        args.push(path);

        // Add the queries
        let queries = self.queries.to_string();
        args.push(queries.as_str());

        Ok(args.iter().map(|s| s.to_string()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CodeQL, CodeQLDatabase};

    fn init_codeql() -> (CodeQL, CodeQLDatabase) {
        let codeql = CodeQL::default();
        let database = CodeQLDatabase::init()
            .name("test")
            .language("javascript")
            .source(PathBuf::from("/path/to/source"))
            .build()
            .unwrap();
        (codeql, database)
    }

    #[test]
    fn test_codeql_create() {
        let (codeql, database) = init_codeql();
        let cmd = CodeQLDatabaseHandler::new(&database, &codeql)
            .sarif(PathBuf::from("test.sarif"))
            .create_cmd()
            .unwrap();

        assert_eq!(cmd.len(), 9);
        assert_eq!(cmd[0], "database");
        assert_eq!(cmd[1], "create");
        assert_eq!(cmd[2], "-l");
        assert_eq!(cmd[3], "javascript");
        assert_eq!(cmd[4], "-s");
        assert_eq!(cmd[5], "/path/to/source");
        // Summary enabled by default
        assert_eq!(cmd[6], "--print-diagnostics-summary");
        assert_eq!(cmd[7], "--print-metrics-summary");
        assert_eq!(cmd[8], database.path().to_str().unwrap());
    }

    #[test]
    fn test_codeql_create_threat_model() {
        let (codeql, database) = init_codeql();
        let cmd = CodeQLDatabaseHandler::new(&database, &codeql)
            .threat_model("test".to_string())
            .create_cmd()
            .unwrap();

        assert_eq!(cmd.len(), 11);
        assert_eq!(cmd[6], "--threat-models");
        assert_eq!(cmd[7], "test");

        let cmd = CodeQLDatabaseHandler::new(&database, &codeql)
            .threat_models(vec!["test".to_string(), "test2".to_string()])
            .create_cmd()
            .unwrap();

        assert_eq!(cmd.len(), 11);
        assert_eq!(cmd[6], "--threat-models");
        assert_eq!(cmd[7], "test,test2");

        let cmd = CodeQLDatabaseHandler::new(&database, &codeql)
            .disable_default_threat_model()
            .create_cmd()
            .unwrap();
        assert_eq!(cmd.len(), 11);
        assert_eq!(cmd[6], "--threat-models");
        assert_eq!(cmd[7], "!default");
    }

    #[test]
    fn test_codeql_create_pack_models() {
        let (codeql, database) = init_codeql();
        let cmd = CodeQLDatabaseHandler::new(&database, &codeql)
            .model_packs(vec!["test".to_string(), "test2".to_string()])
            .create_cmd()
            .unwrap();

        assert_eq!(cmd.len(), 11);
        assert_eq!(cmd[6], "--model-packs");
        assert_eq!(cmd[7], "test,test2");
    }

    #[test]
    fn test_codeql_analysis() {
        let (codeql, database) = init_codeql();
        let cmd = CodeQLDatabaseHandler::new(&database, &codeql)
            .sarif("test.sarif")
            .analyze_cmd()
            .unwrap();

        assert_eq!(cmd.len(), 8);
        assert_eq!(cmd[0], "database");
        assert_eq!(cmd[1], "analyze");
        assert_eq!(cmd[2], "--output");
        assert_eq!(cmd[3], "test.sarif");
        assert_eq!(cmd[4], "--format");
        assert_eq!(cmd[5], "sarif-latest");
        assert_eq!(cmd[6], database.path().to_str().unwrap());
    }

    #[test]
    fn test_codeql_analysis_suites() {
        let (codeql, database) = init_codeql();
        let cmd = CodeQLDatabaseHandler::new(&database, &codeql)
            .suite("javascript")
            .analyze_cmd()
            .unwrap();
        assert_eq!(cmd.len(), 8);
        assert_eq!(cmd[7], "codeql/javascript-queries");

        let cmd = CodeQLDatabaseHandler::new(&database, &codeql)
            .suite("security-extended")
            .analyze_cmd()
            .unwrap();
        assert_eq!(cmd.len(), 8);
        assert_eq!(
            cmd[7],
            "codeql/javascript-queries:codeql-suites/javascript-security-extended.qls"
        );
    }

    #[test]
    fn test_codeql_analysis_queries() {
        let (codeql, database) = init_codeql();
        let cmd = CodeQLDatabaseHandler::new(&database, &codeql)
            .queries("codeql/javascript-queries@0.9.0")
            .analyze_cmd()
            .unwrap();
        assert_eq!(cmd.len(), 8);
        assert_eq!(cmd[7], "codeql/javascript-queries@0.9.0");

        let cmd = CodeQLDatabaseHandler::new(&database, &codeql)
            .queries("codeql/javascript-queries@0.9.0:codeql-suites/javascript-code-scanning.qls")
            .analyze_cmd()
            .unwrap();
        assert_eq!(cmd.len(), 8);
        assert_eq!(
            cmd[7],
            "codeql/javascript-queries@0.9.0:codeql-suites/javascript-code-scanning.qls"
        );
    }
}
