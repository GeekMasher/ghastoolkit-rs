use clap::{Parser, Subcommand};

use console::style;
use ghastoolkit::{CodeQLDatabases, GHASError, GitHub, Repository};

pub const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub const BANNER: &str = r"
 _____  _   _   ___   _____ _____           _ _    _ _   
|  __ \| | | | / _ \ /  ___|_   _|         | | |  (_) |  
| |  \/| |_| |/ /_\ \\ `--.  | | ___   ___ | | | ___| |_ 
| | __ |  _  ||  _  | `--. \ | |/ _ \ / _ \| | |/ / | __|
| |_\ \| | | || | | |/\__/ / | | (_) | (_) | |   <| | |_ 
 \____/\_| |_/\_| |_/\____/  \_/\___/ \___/|_|_|\_\_|\__|";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[clap(long, help = "Enable Debugging", default_value_t = false)]
    pub debug: bool,

    #[clap(long, help = "Disable Banner", default_value_t = false)]
    pub disable_banner: bool,

    #[clap(long, env, help = "GitHub Token")]
    pub github_token: Option<String>,

    #[clap(long, env, help = "GitHub Repository")]
    pub github_repository: Option<String>,

    #[clap(long, env, help = "GitHub Owner (Organization / User)")]
    pub github_owner: Option<String>,

    #[clap(
        long,
        env,
        help = "GitHub Instance",
        default_value_t = String::from("https://github.com")
    )]
    pub github_instance: String,

    #[clap(long, env, help = "GitHub Branch")]
    pub github_branch: Option<String>,

    #[clap(long, env = "GITHUB_REF", help = "GitHub Reference")]
    pub github_reference: Option<String>,

    #[clap(subcommand)]
    pub commands: Option<ArgumentCommands>,
}

#[derive(Subcommand, Debug)]
pub enum ArgumentCommands {
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

    Codescanning {
        #[clap(short, long, help = "Audit Mode", default_value_t = false)]
        audit: bool,
    },

    Codeql {
        #[clap(long, env, help = "Path to CodeQL")]
        codeql_path: Option<String>,

        #[clap(long, env, help = "CodeQL Database / Databases Root Path", default_value_t = default_codeql_path())]
        codeql_databases: String,

        #[clap(short, long, help = "List CodeQL Databases")]
        list: bool,

        #[clap(short, long, help = "Repository mode")]
        repo: bool,

        #[clap(long, help = "List CodeQL Languages")]
        languages: bool,

        #[clap(long, help = "CodeQL Language")]
        language: Option<String>,

        #[clap(long, help = "CodeQL Suite")]
        suite: Option<String>,

        #[clap(short, long, help = "Download CodeQL Databases")]
        download: bool,

        #[clap(long, help = "Number of Threads / CPU Cores to use")]
        threads: Option<usize>,

        #[clap(long, help = "Amount of Memory / RAM to use in MB")]
        ram: Option<usize>,
    },
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
            .instance(self.github_instance.clone().as_str())
            .owner(self.github_owner.clone().unwrap_or_default().as_str())
            .token(self.github_token.clone().unwrap_or_default().as_str())
            .build()
            .expect("Failed to build GitHub client")
    }

    pub fn repository(&self) -> Result<Repository, GHASError> {
        Repository::init()
            .repo(self.github_repository.clone().unwrap_or_default().as_str())
            .reference(self.github_reference.clone().unwrap_or_default().as_str())
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

    if !arguments.disable_banner {
        println!(
            "{}    {} - v{}\n",
            style(BANNER).green(),
            style(AUTHOR).red(),
            style(VERSION_NUMBER).blue()
        );
    }

    arguments
}
