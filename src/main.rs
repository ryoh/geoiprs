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
    println!("City: {:?}", city.city.unwrap().names.unwrap()["ja"]);

    let country = city.country.unwrap();
    println!("ISO Code: {:?}", country.iso_code.unwrap());

    let continent = city.continent.unwrap();
    println!("Continent Code: {:?}", continent.code.unwrap());

    Ok(())
}
