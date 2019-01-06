extern crate maxminddb;
extern crate structopt;

use std::io::{self, Write};
use std::process;

use quicli::prelude::*;
use structopt::StructOpt;

use std::net::IpAddr;

use maxminddb::{geoip2, Reader};

#[derive(Debug, StructOpt)]
struct Cli {
    /// MaxmindDB file path
    #[structopt(short = "f", long = "file", default_value = "./GeoLite2-City.mmdb")]
    file: String,
    /// Search object
    #[structopt(name = "IPADDRESS")]
    ipaddrs: Vec<String>,
}

#[derive(Debug, Serialize)]
struct OutputData {
    address: String,
    iso_code: String,
    name: String,
}

impl OutputData {
    fn new(address: &IpAddr, iso_code: &str, name: &str) -> OutputData {
        OutputData {
            address: address.to_string(),
            iso_code: iso_code.to_string(),
            name: name.to_string(),
        }
    }

    fn print(&self, handle: &mut io::BufWriter<io::StdoutLock>) {
        writeln!(
            handle,
            "GeoIP Country Edition: {}, {}, ({})",
            self.iso_code, self.name, self.address
        )
        .unwrap();
    }
}

fn geoiplookup(reader: &maxminddb::Reader<Vec<u8>>, address: &IpAddr) -> OutputData {
    // Lookup GeoIP data
    let city: geoip2::City = match reader.lookup(*address) {
        Ok(data) => data,
        Err(_) => return OutputData::new(address, "XX", "Unknown country"),
    };

    // Country base
    let country = city.country.expect("Get error country variable");

    // ISO Country code
    let iso_code = country.iso_code.unwrap_or("O1".to_string());

    // Country name
    let names = country.names.unwrap();
    let name = names.get("en").unwrap();

    OutputData::new(&address, &iso_code, &name)
}

fn main() -> CliResult {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    let args = Cli::from_args();

    // Read ipv4 address
    let ipaddrs: Vec<IpAddr> = args.ipaddrs.into_iter().flat_map(|i| i.parse()).collect();

    // MaxmindDB read.
    let mmdb = args.file;
    let reader = match Reader::open_readfile(mmdb) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Could not open MaxMind DB file.");
            process::exit(1);
        }
    };

    // Get output data
    for ipaddr in ipaddrs {
        let data = geoiplookup(&reader, &ipaddr);

        // print
        data.print(&mut handle);
    }

    Ok(())
}
