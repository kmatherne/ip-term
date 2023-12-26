use anyhow::Result;
use clap::Parser;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    ip_address: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_base = "http://ip-api.com/json/";
    let args = Cli::parse();
    let ip = args.ip_address;
    println!("IP Address: {:?}", ip);

    if ip.parse::<Ipv4Addr>().is_ok() {
        println!("Is an Ipv4");
    } else if ip.parse::<Ipv6Addr>().is_ok() {
        println!("Is an Ipv6");
    } else {
        println!("Did not provide a valid IP address!");
    }

    let req_url = format!("{}{}", api_base, ip);
    println!("Sending req to {:?}", req_url);

    let response = reqwest::get(&req_url).await?.text().await?;
    println!("Response: {}", response);
    Ok(())
}
