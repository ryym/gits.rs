extern crate gits;
extern crate getopts;

use std::process;
use std::path::Path;
use std::env;
use getopts::{Options, Matches};
use gits::list_repos;

fn main() {
    let opts = parse_opts(&env::args().collect());
    match opts {
        Err(message) => {
            println!("{}", message);
            return;
        }
        Ok(opts) => {
            let path_name = &opts.free[0];
            let dir = Path::new(path_name);

            if !dir.exists() || dir.is_file() {
                println!("Directory does not exist: {}", path_name);
                process::exit(1);
            }

            let exit_code = print_repos(dir);
            process::exit(exit_code);
        }
    }
}

fn parse_opts(args: &Vec<String>) -> Result<Matches, String> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this usage.");

    let matches = opts.parse(&args[1..]).map_err(|e| e.to_string())?;

    if !matches.opt_present("h") && matches.free.len() == 1 {
        Ok(matches)
    } else {
        let usage = &format!("Usage: {} [options] <path>", &args[0]);
        Err(opts.usage(usage))
    }
}

fn print_repos(dir: &Path) -> i32 {
    match list_repos(dir, ".git") {
        Ok(repos) => {
            for p in repos {
                println!("{}", p);
            }
            0
        }
        Err(e) => {
            println!("ERR: {}", e.to_string());
            1
        }
    }
}
