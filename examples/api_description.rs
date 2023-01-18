use anyhow::Result;

use fritz_tr064_upnp::Gateway;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let gateway = Gateway::discover(Default::default()).await?;

    let response = gateway.api_description().await?;

    println!("{:#?}", response);

    Ok(())
}
