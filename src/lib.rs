pub mod parser;
pub mod banner;

pub const BAD: &str = "\x1b[1;91m[-]\x1b[0m";
pub const GOOD: &str = "\x1b[1;32m[+]\x1b[0m";
pub const INFO: &str = "\x1b[1;93m[!]\x1b[0m";
pub const CHECK: &str = "\x1b[1;32m[✓]\x1b[0m";
// pub const STATUS_OK: &str = "\x1b[1;32m200 OK\x1b[0m";
// pub const STATUS_UNAUTHORIZED: &str = "\x1b[1;93m401 UNAUTHORIZED\x1b[0m";
// pub const STATUS_NOTFOUND: &str = "\x1b[1;91m404 NOT FOUND\x1b[0m";
// pub const STATUS_FORBIDDEN: &str = "\x1b[1;93m403 FORBIDDEN\x1b[0m";
// pub const STATUS_MOVED_PERMANENTLY: &str = "\x1b[1;93m301 MOVED PERMANENTLY\x1b[0m";
// pub const STATUS_TEMP_REDIRECT: &str = "\x1b[1;93m307 TEMPORARY REDIRECT\x1b[0m";
pub const VERSION: &str = "0.0.1";
