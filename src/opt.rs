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
    /// Base directory in which to search.
    #[arg(short = 'd', long = "dir",
        default_value_t = String::from("."))]
    base: String,
    /// The pattern(s) to match file paths against.
    pattern: Vec<String>,
    /// Use regex (instead of glob) matching.
    #[arg(short, long, default_value_t = false)]
    regex: bool,
    /// Print absolute paths. [default: relative to BASE]
    #[arg(short, long)]
    absolute: bool,
    /// Match any part of the path, not just the filename.
    #[arg(short, long)]
    full: bool,
    /// Show access errors (default is to ignore them).
    #[arg(short, long)]
    errors: bool,
}

/// Options derived from [`OptArgs`] to be usable to the rest of
/// the program.
#[derive(Default)]
pub struct Opts {
    /// Set of filename patterns against which to match.
    pub patterns: RegexSet,
    /// Base directory from which to start searching.
    pub base: PathBuf,
    /// Whether to display aboslute (or relative) path names.
    pub absolute: bool,
    /// Whether to match on _any_ part of the path (not just
    /// the final element).
    pub full: bool,
    /// Show errors (default is to ignore them because they are usually
    /// just permissions errors).
    pub errors: bool,
}

impl Opts {
    pub fn new() -> Result<Opts, String> {
        let oa = OptArgs::parse();
        if oa.pattern.is_empty() {
            return Err("you must specify at least one pattern".into());
        }

        let mut opts = Opts::default();

        let oa_strs: Vec<String> = if oa.regex {
            oa.pattern
        } else {
            oa.pattern
                .iter()
                .map(|pat| Glob::new(pat))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("{}", &e))?
                .into_iter()
                .map(|g| String::from(g.regex()))
                .collect()
        };

        let patterns = RegexSet::new(&oa_strs).map_err(|e| format!("{}", &e))?;

        opts.patterns = patterns;
        opts.base = PathBuf::from(oa.base);
        opts.absolute = oa.absolute;
        opts.full = oa.full;
        opts.errors = oa.errors;

        Ok(opts)
    }
}
