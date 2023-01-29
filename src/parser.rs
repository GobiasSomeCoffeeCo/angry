use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command, ValueHint};

pub fn cli() -> Command {
    let app = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!());

    /////////////////////////////////////////////////////////////////////
    // group - target selection
    /////////////////////////////////////////////////////////////////////
    let app = app
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .required_unless_present_any(["stdin", "resume_from"])
                .help_heading("Target selection")
                .value_name("URL")
                .use_value_delimiter(true)
                .value_hint(ValueHint::Url)
                .help("The target URL (required, unless [--stdin || --resume-from] used)"),
        )
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .help_heading("Target selection")
                .num_args(0)
                .help("Read url(s) from STDIN")
                .conflicts_with("url")
        )
        .arg(
            Arg::new("resume_from")
                .long("resume-from")
                .value_hint(ValueHint::FilePath)
                .value_name("STATE_FILE")
                .help_heading("Target selection")
                .help("State file from which to resume a partially complete scan (ex. --resume-from ferox-1606586780.state)")
                .conflicts_with("url")
                .num_args(1),
        );
    app
}
