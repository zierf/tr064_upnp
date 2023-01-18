use anyhow::Result;

use fritz_tr064_upnp::Gateway;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let gateway = Gateway::discover(Default::default()).await?;

    let overview = gateway.overview().await?;

    println!("{:#?}", overview);

    Ok(())
}
