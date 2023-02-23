use std::{collections::HashMap, io::Write};

use crate::{banner::Banner, CHECK, VERSION};

#[derive(Debug, Clone)]
pub struct Config {
    pub status_codes: Vec<u16>,
    pub exclude_status_codes: Option<Vec<u16>>,
    pub url: String,
    pub wordlist: String,
    pub threads: usize,
    pub headers: HashMap<String, String>,
    pub insecure: bool,
    pub proxy: Option<String>,
    pub timeout: usize,
    pub redirects: bool,
    pub user_agent: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            status_codes: vec![200, 204, 301, 302, 307, 308, 401, 403, 405],
            url: "https://www.example.com".to_string(),
            wordlist: "directories.txt".to_string(),
            threads: 50,
            exclude_status_codes: None,
            headers: HashMap::new(),
            insecure: false,
            proxy: None,
            timeout: 7,
            redirects: false,
            user_agent: String::new(),
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
        let redirects = Banner::new(CHECK, "Follow Redirects", &self.redirects.to_string());
        let timeout = Banner::new(CHECK, "Timeout", &self.timeout.to_string());

        writeln!(&mut writer, "{}", self.header())?;
        writeln!(&mut writer, "{}", url)?;
        writeln!(&mut writer, "{}", wordlist)?;

        // This prints "either excluded or included Status Codes since these two features cannot be used together
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

        writeln!(&mut writer, "{}", redirects)?;
        writeln!(&mut writer, "{} second's", timeout)?;

        writeln!(&mut writer, "{}", self.footer())?;

        Ok(())
    }
}
