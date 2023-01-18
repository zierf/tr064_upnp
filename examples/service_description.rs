use anyhow::Result;
use url::Host;

use fritz_tr064_upnp::{Gateway, Scheme};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let gateway: Gateway = Gateway::builder()
        .host(Host::parse("fritz.box").unwrap())
        .port(49000)
        .scheme(Scheme::HTTP)
        .build();

    let dummy_description = gateway.service_description("/any.xml").await?;
    let response = gateway.service_description("/igdicfgSCPD.xml").await?;

    println!("{:#?}", dummy_description);
    println!("{:#?}", response);

    Ok(())
}
