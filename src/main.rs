use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Search object
    ipaddr: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    println!("{:?}", args);

    Ok(())
}
