use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};
#[macro_use]
extern crate prettytable;
use prettytable::{format::Alignment, Cell, Row};

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    ip_address: String,
}

#[derive(Deserialize, Debug)]
struct IpInfo {
    query: String,
    country: String,
    #[serde(rename = "regionName")]
    region_name: String,
    city: String,
    zip: String,
    isp: String,
    org: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_base = "http://ip-api.com/json/";
    let args = Cli::parse();
    let ip = args.ip_address;

    if ip.parse::<Ipv4Addr>().is_ok() {
    } else if ip.parse::<Ipv6Addr>().is_ok() {
    } else {
        println!("Did not provide a valid IP address!");
    }

    let req_url = format!("{}{}", api_base, ip);

    let response = reqwest::get(&req_url).await?;
    let ip_info: IpInfo = response.json().await?;

    let mut table = table!(
        ["IP", &ip_info.query],
        ["Country", &ip_info.country],
        ["Region", &ip_info.region_name],
        ["City", &ip_info.city],
        ["Zip", &ip_info.zip],
        ["ISP", &ip_info.isp],
        ["Organization", &ip_info.org]
    );
    table.set_titles(Row::new(vec![Cell::new_align(
        "IP Info",
        Alignment::CENTER,
    )
    .with_hspan(2)]));

    table.printstd();

    Ok(())
}
