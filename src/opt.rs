/**!
Argument parsing and configutation.
*/
use std::path::PathBuf;

use clap::Parser;
use globset::Glob;
use regex::bytes::RegexSet;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct OptArgs {
    /// The pattern(s) to match file paths against.
    pattern: Vec<String>,
    /// Use regex (instead of glob) matching.
    #[arg(short, long, default_value_t = false)]
    regex: bool,
    /// Base directory in which to search.
    #[arg(short, long)]
    base: Option<PathBuf>,
}

#[derive(Default)]
pub struct Opts {
    pub patterns: RegexSet,
    pub base: PathBuf,
}

impl Opts {
    pub fn new() -> Result<Opts, String> {
        let oa = OptArgs::parse();
        let mut opts = Opts::default();

        let oa_strs: Vec<String> = if oa.regex {
            oa.pattern
        } else {
            oa.pattern.iter()
                .map(|pat| Glob::new(pat))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("{}", &e))?
                .into_iter()
                .map(|g| String::from(g.regex()))
                .collect()
        };

        let patterns = RegexSet::new(&oa_strs).map_err(|e| format!(
            "{}", &e
        ))?;

        opts.patterns = patterns;
        opts.base = oa.base.unwrap_or_else(|| PathBuf::from("."));

        Ok(opts)
    }
}