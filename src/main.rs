
mod opt;

use std::{
    error::Error,
    io::{stdout, Write},
};

use bstr::ByteSlice;
use walkdir::WalkDir;

use opt::Opts;

static NEWLINE: &[u8] = b"\n";

fn do_the_thing(opts: Opts) -> Result<(), Box<dyn Error>> {
    for p in WalkDir::new(&opts.base)
        .follow_links(false)
    {
        let p = match p {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}", &e);
                continue;
            }
        };

        let fname = match p.path().file_name() {
            Some(fname) => fname,
            None => continue,
        };

        let bytes = match <[u8]>::from_os_str(fname) {
            None => continue,
            Some(bytes) => bytes,
        };

        if opts.patterns.is_match(bytes) {
            let bytes = match <[u8]>::from_os_str(p.path().as_os_str()) {
                None => continue,
                Some(bytes) => bytes,
            };

            let mut stdout = stdout();
            stdout.write_all(bytes)?;
            stdout.write_all(NEWLINE)?;
        }
    }

    Ok(())
}

fn wrapped_main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::new()?;
    do_the_thing(opts)
}

fn main() {
    if let Err(e) = wrapped_main() {
        eprintln!("{}", &e);
        std::process::exit(1);
    }
}
