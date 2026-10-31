#![forbid(unsafe_code)]

use droidlink_local_policy::LocalEndpoint;
use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if arguments.len() != 1 {
        eprintln!("Usage: droidlink-local-policy <literal-ip:port>");
        return ExitCode::from(2);
    }
    match LocalEndpoint::parse(&arguments[0]) {
        Ok(_) => {
            println!("ACCEPTED by address policy. No network connection was made.");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("REJECTED: {error}");
            ExitCode::from(1)
        }
    }
}
