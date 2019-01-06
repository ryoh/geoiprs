extern crate maxminddb;

use std::io::{self, Write};

use quicli::prelude::*;
use structopt::StructOpt;

use std::net::IpAddr;

use maxminddb::{geoip2, Reader};

#[derive(Debug, StructOpt)]
struct Cli {
    /// Search object
    ipaddr: String,
}

fn main() -> CliResult {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    let args = Cli::from_args();

    // Read ipv4 address
    let ipaddr: IpAddr = args.ipaddr.parse().expect("Parse error IP Address");

    // MaxmindDB read.
    let mmdb = "GeoLite2-City.mmdb";
    let reader = Reader::open_readfile(mmdb).expect("Open error");

    // Loopup Geoip data
    let city: geoip2::City = reader.lookup(ipaddr).expect("Lookup error");
    //println!("{:#?}", city);
    let country = city.country.expect("Get error country variable");

    // ISO Country code
    let iso_code = country.iso_code.unwrap_or("O1".to_string());

    // Country name
    let names = country.names.unwrap();
    let name = names.get("en").unwrap().to_string();

    // City code
    let _continet = city.continent.expect("Get error continent variable");

    writeln!(handle, "GeoIP Country Edition: {}, {}", iso_code, name).unwrap();

    Ok(())
}
