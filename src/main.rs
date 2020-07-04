use maxminddb::geoip2;
use std::net::IpAddr;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    // test parameters
    let mmdb_path = "/var/lib/GeoIP/GeoLite2-City.mmdb";
    let test_ipaddr = "52.25.53.18";

    // open geoip database
    let reader = maxminddb::Reader::open_readfile(&mmdb_path)?;

    // lookup geoip data
    let ipaddr: IpAddr = FromStr::from_str(test_ipaddr)?;
    let city: geoip2::City = reader.lookup(ipaddr)?;

    println!("{:#?}", city.continent);

    Ok(())
}
