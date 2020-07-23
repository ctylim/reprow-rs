use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    pub log_level: String,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub rep_col: String,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config::default();
        let version: String =
            env!("CARGO_PKG_VERSION").to_string() + "\ncommit " + env!("GIT_COMMIT_HASH") + "\ncommit-date " + env!("GIT_COMMIT_DATE");
        let matches = App::new(env!("CARGO_PKG_NAME"))
            .long_version(version.as_ref())
            .author("ctylim")
            .about("reprow-rs")
            .arg(
                Arg::with_name("log")
                    .long("log")
                    .value_name("off|error|warn|info|debug|trace")
                    .help("Sets log level. (default: off)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("src")
                    .long("src")
                    .value_name("PATH")
                    .help("Sets source file path. If not set, source sets to stdin. (default: stdin)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("dst")
                    .long("dst")
                    .value_name("PATH")
                    .help("Sets destination file path. If not set, destination sets to stdout. (default: stdout)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("col")
                    .long("col")
                    .value_name("STRING")
                    .help("Sets column which specifies times to repeat rows.\nRows are skipped if the column is non-positive integer.")
                    .takes_value(true),
            )
            .get_matches();
        let parse_failed = |a: &str, s: &str| format!("Parse failed in command argument {}: {}", a, s);
        if let Some(log_level) = matches.value_of("log") {
            config.log_level = log_level.parse().expect(&parse_failed("log_level", log_level));
        }
        if let Some(source) = matches.value_of("src") {
            config.source = Some(source.parse().expect(&parse_failed("destination", source)));
        }
        if let Some(destination) = matches.value_of("dst") {
            config.destination = Some(destination.parse().expect(&parse_failed("destination", destination)));
        }
        if let Some(rep_col) = matches.value_of("col") {
            config.rep_col = rep_col.parse().expect(&parse_failed("rep_col", rep_col));
        }
        config
    }

    pub fn show(&self) {
        info!("{:?}", self);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: "off".to_string(),
            source: None,
            destination: None,
            rep_col: "".to_string(),
        }
    }
}
