use std::{
    env,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    time::Duration,
};

use anyhow::Result;

use fritz_tr064_upnp::{gateway::BROADCAST_IPV4, gateway::BROADCAST_IPV6, Gateway, SearchOptions};

#[tokio::main(flavor = "current_thread")]
/// Run with `RUST_BACKTRACE=1 cargo run --example gateway_discovery v6`
/// to run discovery over IPv6.
async fn main() -> Result<()> {
    let ip_version = env::args().nth(1).unwrap_or_else(|| "v4".into());

    let bind_address = if ip_version == "v6" {
        IpAddr::V6(Ipv6Addr::UNSPECIFIED)
    } else {
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
    };

    let broadcast_address = (match bind_address {
        IpAddr::V6(_) => BROADCAST_IPV6,
        IpAddr::V4(_) => BROADCAST_IPV4,
    })
    .parse::<SocketAddr>()
    .unwrap();

    let options = SearchOptions {
        bind_address: SocketAddr::new(bind_address, 0),
        broadcast_address,
        timeout: Some(Duration::from_secs(2)),
    };

    // there is also a default implementation for SearchOptions (using IPv4)
    // let options: SearchOptions = Default::default();

    let gateway = Gateway::discover(options).await?;

    println!("{:#?}", gateway);

    Ok(())
}
