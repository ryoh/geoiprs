use maxminddb::geoip2;
use std::net::IpAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    ipaddr: IpAddr,
}

fn main() -> anyhow::Result<()> {
    // get parameters
    let opt = Opt::from_args();

    // open geoip database
    let mmdb_path = "/var/lib/GeoIP/GeoLite2-City.mmdb";
    let reader = maxminddb::Reader::open_readfile(&mmdb_path)?;

    // lookup geoip data
    let city: geoip2::City = reader.lookup(opt.ipaddr)?;

    print!("GeoIP Country: ");
    match city.country {
        Some(country) => {
            let code = country.iso_code.unwrap_or("None");
            let name = match country.names {
                Some(names) => names.get("en").unwrap_or(&"None"),
                None => "None",
            };
            println!("{}, {}", code, name);
        }
        None => println!("None"),
    }

    Ok(())
}
