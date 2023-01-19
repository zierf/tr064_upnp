use anyhow::Result;

use fritz_tr064_upnp::{
    gateway::{DEFAULT_HOSTNAME, DEFAULT_HOSTPORT, DEFAULT_ROOT_URL},
    Gateway, Scheme,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let gateway_builder = Gateway::builder()
        .host(DEFAULT_HOSTNAME)
        .port(DEFAULT_HOSTPORT)
        .scheme(Scheme::HTTP)
        .root_url(DEFAULT_ROOT_URL)
        .seal();

    let gateway = gateway_builder.build()?;

    let dummy_description = gateway.service_description("/any.xml").await?;
    let response = gateway.service_description("/igdicfgSCPD.xml").await?;

    println!("{:#?}", dummy_description);
    println!("{:#?}", response);

    Ok(())
}
