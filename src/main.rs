mod access_log;
mod args;
mod error;
mod fs;
mod server;

use exitcode;
use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let code = match server::serve(args) {
        Ok(_) => exitcode::OK,
        Err(err) => {
            eprintln!("{}", err);
            exitcode::USAGE
        }
    };
    exit(code)
}
