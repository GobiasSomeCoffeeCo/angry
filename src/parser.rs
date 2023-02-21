use std::io::Write;

use crate::{banner::Banner, CHECK, VERSION};

use clap::{Parser, Subcommand};

#[derive(Debug, Clone)]
pub struct Config {
    pub status_codes: Vec<u16>,
    pub exclude_status_codes: Option<Vec<u16>>,
    pub url: String,
    pub wordlist: String,
    pub threads: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            status_codes: vec![200, 204, 301, 302, 307, 308, 401, 403, 405],
            url: "https://www.example.com".to_string(),
            wordlist: "directories.txt".to_string(),
            threads: 50,
            exclude_status_codes: None,
        }
    }
}

impl Config {
    pub fn header(&self) -> String {
        let banner = format!(
            r#"
   __  __  _  __ _____   __
  /  \|  \| |/ _] _ \ `v' /
 | /\ | | ' | [/\ v /`. .' 
 |_||_|_|\__|\__/_|_\ !_! {} by Gobias Industries...
 Version {}"#,
            '\u{1F620}', VERSION,
        );
        let top = "───────────────────────────┬──────────────────────";
        format!("{banner}\n{top}")
    }

    pub fn footer(&self) -> String {
        
        "───────────────────────────┴──────────────────────".to_string()
    }

    pub fn print_banner<W>(&self, mut writer: W) -> anyhow::Result<()>
    where
        W: Write,
    {
        let mut exclude = Vec::new();
        let mut included = Vec::new();

        let url = Banner::new(CHECK, "Target", &self.url);
        let wordlist = Banner::new(CHECK, "Wordlist", &self.wordlist);

        writeln!(&mut writer, "{}", self.header())?;
        writeln!(&mut writer, "{}", url)?;
        writeln!(&mut writer, "{}", wordlist)?;

        // This prints "either/or" since these two features cannot be used together
        if let Some(excluded) = &self.exclude_status_codes {
            for code in excluded {
                exclude.push(code.to_string())
            }
            let ex_status_codes = Banner::new(
                CHECK,
                "Excluded Status Codes",
                &format!("[{}]", exclude.join(", ")),
            );
            writeln!(&mut writer, "{}", ex_status_codes)?;
        } else {
            for code in &self.status_codes {
                included.push(code.to_string())
            }
            let status_codes =
                Banner::new(CHECK, "Status Codes", &format!("[{}]", included.join(", ")));
            writeln!(&mut writer, "{}", status_codes)?;
        }

        writeln!(&mut writer, "{}", self.footer())?;

        Ok(())
    }
}

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "A Content Discovery Tool written in Rust "
)]
pub struct Cli {
    /// Target URL
    #[arg(short, long, value_name = "https://www.<target>.com")]
    pub url: String,

    /// Path to the wordlist
    #[arg(short, long, default_value = "directories.txt", value_name = "FILE")]
    pub wordlist: String,

    /// Number of threads.
    #[arg(short, long, default_value_t = 50, value_name = "NUMBER")]
    pub threads: usize,

    /// Status Codes to include (allow list) (default: 200 204 301 302 307 308 401 403 405)
    #[arg(short, long, use_value_delimiter = true, value_parser, num_args = 1.., conflicts_with = "exclude_status_codes", action = clap::ArgAction::Append, value_name = "STATUS_CODE")]
    pub status_codes: Option<Vec<u16>>,

    /// Status Codes to exclude aka inverse of --status-codes (returns all status codes except the ones passed)
    #[arg(short, long, use_value_delimiter = true, value_parser, num_args = 1.., conflicts_with = "status_codes", action = clap::ArgAction::Append, value_name = "STATUS_CODE")]
    pub exclude_status_codes: Option<Vec<u16>>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

pub fn cli_parse() -> Config {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    let mut config = Config {
        url: cli.url,
        wordlist: cli.wordlist,
        threads: cli.threads,
        ..Default::default()
    };

    if let Some(status) = cli.exclude_status_codes {
        config.exclude_status_codes = Some(status)
    }

    if let Some(status) = cli.status_codes {
        config.status_codes = status
    }

    // if let Some(Config_path) = cli.wordlist {
    //     println!("Value for Config: {}", Config_path.display());
    // }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => (),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }
    config
    // Continued program logic goes here...
}
