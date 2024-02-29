use clap::{Parser, Subcommand};

use console::style;
use ghastoolkit::{GitHub, Repository};

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
    CodeScanning {
        #[clap(short, long, help = "Audit Mode", default_value_t = false)]
        audit: bool,
    },
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

    pub fn repository(&self) -> Repository {
        Repository::init()
            .repo(self.github_repository.clone().unwrap_or_default().as_str())
            .reference(self.github_reference.clone().unwrap_or_default().as_str())
            .build()
            .expect("Failed to build Repository")
    }
}

pub fn init() -> Arguments {
    // Load .env file if it exists
    dotenv::dotenv().ok();

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
