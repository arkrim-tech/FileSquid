//! Stand-alone filesquid-metadata CLI.

use filesquid_metadata::cli::run;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(code) => process::exit(code),
        Err(err) => {
            eprintln!("{err}");
            process::exit(2);
        }
    }
}
