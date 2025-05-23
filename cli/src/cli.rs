use clap::{Parser, Subcommand, builder::styling};
use console::style;
use ghastoolkit::{CodeQLDatabases, GHASError, GitHub, Repository};
use std::{fmt::Display, path::PathBuf};

pub const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub const BANNER: &str = r"
 _____  _   _   ___   _____ _____           _ _    _ _   
|  __ \| | | | / _ \ /  ___|_   _|         | | |  (_) |  
| |  \/| |_| |/ /_\ \\ `--.  | | ___   ___ | | | ___| |_ 
| | __ |  _  ||  _  | `--. \ | |/ _ \ / _ \| | |/ / | __|
| |_\ \| | | || | | |/\__/ / | | (_) | (_) | |   <| | |_ 
 \____/\_| |_/\_| |_/\____/  \_/\___/ \___/|_|_|\_\_|\__|";

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::BrightBlue.on_default().bold())
    .usage(styling::AnsiColor::White.on_default().bold())
    .literal(styling::AnsiColor::BrightGreen.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, propagate_version = true, styles = STYLES)]
pub struct Arguments {
    /// Output Format
    #[clap(short, long, value_enum, default_value_t = OutputFormat::Std)]
    pub format: OutputFormat,

    /// Output File
    #[clap(short = 'O', long)]
    pub output: Option<PathBuf>,

    /// GitHub Settings
    #[clap(flatten)]
    pub github: GitHubGroup,

    /// Enable Debugging
    #[clap(long, env, default_value_t = false)]
    pub debug: bool,

    /// Disable Banner
    #[clap(long, env, default_value_t = false)]
    pub disable_banner: bool,

    /// Subcommands
    #[clap(subcommand)]
    pub commands: Option<ArgumentCommands>,
}

/// Subcommands for the CLI
#[derive(Subcommand, Debug)]
pub enum ArgumentCommands {
    /// Secret Scanning
    Secretscanning {
        /// Secret Scanning Alert State
        #[clap(short, long)]
        state: Option<String>,
        /// Secret Type / Name
        #[clap(short, long)]
        r#type: Option<String>,
        /// Validity
        #[clap(short, long)]
        validity: Option<String>,
        /// Short Links
        #[clap(short, long, default_value_t = false)]
        links: bool,
    },
    /// Code Scanning
    Codescanning {
        #[clap(short, long, help = "Audit Mode", default_value_t = false)]
        audit: bool,
    },
    /// CodeQL
    Codeql {
        /// List CodeQL Databases
        #[clap(short, long)]
        list: bool,
        /// Download CodeQL Database
        #[clap(short, long)]
        download: bool,
        /// Repository Mode
        #[clap(short, long)]
        repo: bool,
        /// List CodeQL Languages
        #[clap(short = 'L', long)]
        languages: bool,

        /// CodeQL Path
        #[clap(long, env)]
        codeql_path: Option<String>,
        /// CodeQL Databases Path Root Path
        #[clap(long, env, default_value_t = default_codeql_path())]
        codeql_databases: String,
        /// CodeQL Language
        #[clap(long)]
        language: Option<String>,
        /// CodeQL Suite
        #[clap(long)]
        suite: Option<String>,
        /// Number of Threads / CPU Cores to use
        #[clap(long)]
        threads: Option<usize>,
        /// Amount of Memory / RAM to use in MB
        #[clap(long)]
        ram: Option<usize>,
    },
}

/// Output Format
#[derive(clap::ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    /// Standard Output
    Std,
    /// JSON Output
    Json,
}

/// GitHub Settings
#[derive(clap::Args, Debug)]
pub struct GitHubGroup {
    /// GitHub Token
    #[clap(short = 't', long, env, hide_possible_values = true)]
    pub github_token: Option<String>,

    /// GitHub Repository which can be in the format of `<repo>` or `<owner>/<repo>`
    #[clap(short = 'r', long, env)]
    pub github_repository: Option<String>,

    /// GitHub Branch Name which can be `<branch>`, `<tag>`, or `<sha>`
    #[clap(short = 'b', long, env)]
    pub github_branch: Option<String>,

    /// GitHub Owner (Organization / User) which can be in the format of `<owner>`
    #[clap(long, env)]
    pub github_owner: Option<String>,

    /// GitHub Enterprise Account Name
    #[clap(long, env)]
    pub github_enterprise: Option<String>,

    /// GitHub Instance URL
    #[clap(
        long,
        env,
        default_value_t = String::from("https://github.com")
    )]
    pub github_instance: String,

    /// GitHub Reference which can be `<ref>` like `refs/heads/<branch>`, `refs/tags/<tag>`, or `refs/pull/<pr>/merge`
    #[clap(long, env = "GITHUB_REF")]
    pub github_reference: Option<String>,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Std => write!(f, "std"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}

fn default_codeql_path() -> String {
    CodeQLDatabases::default_path()
        .to_str()
        .expect("Failed to convert PathBuf to String")
        .to_string()
}

impl Arguments {
    pub fn github(&self) -> GitHub {
        GitHub::init()
            .instance(self.github.github_instance.clone().as_str())
            .enterprise(
                self.github
                    .github_enterprise
                    .clone()
                    .unwrap_or_default()
                    .as_str(),
            )
            .owner(
                self.github
                    .github_owner
                    .clone()
                    .unwrap_or_default()
                    .as_str(),
            )
            .token(
                self.github
                    .github_token
                    .clone()
                    .unwrap_or_default()
                    .as_str(),
            )
            .build()
            .expect("Failed to build GitHub client")
    }

    pub fn repository(&self) -> Result<Repository, GHASError> {
        Repository::init()
            .repo(
                self.github
                    .github_repository
                    .clone()
                    .unwrap_or_default()
                    .as_str(),
            )
            .branch(
                self.github
                    .github_branch
                    .clone()
                    .unwrap_or_default()
                    .as_str(),
            )
            .reference(
                self.github
                    .github_reference
                    .clone()
                    .unwrap_or_default()
                    .as_str(),
            )
            .build()
    }
}

pub fn init() -> Arguments {
    // Load .env file if it exists
    dotenvy::dotenv().ok();

    let arguments = Arguments::parse();

    let log_level = match &arguments.debug {
        false => log::LevelFilter::Info,
        true => log::LevelFilter::Debug,
    };

    env_logger::builder()
        .parse_default_env()
        .filter_level(log_level)
        .init();

    if !arguments.disable_banner && arguments.format == OutputFormat::Std {
        println!(
            "{}    {} - v{}\n",
            style(BANNER).green(),
            style(AUTHOR).red(),
            style(VERSION_NUMBER).blue()
        );
    }

    arguments
}
