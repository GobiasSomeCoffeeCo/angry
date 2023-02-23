pub mod banner;
pub mod client;
pub mod parser;
pub mod config;

pub const BAD: &str = "\x1b[1;91m[-]\x1b[0m";
pub const GOOD: &str = "\x1b[1;32m[+]\x1b[0m";
pub const INFO: &str = "\x1b[1;93m[!]\x1b[0m";
pub const CHECK: &str = "\x1b[1;36m[✓]\x1b[0m";
pub const VERSION: &str = "0.2.0";
