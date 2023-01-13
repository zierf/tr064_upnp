use anyhow::Result;

use fritz_tr064_upnp::{services, UpnpHost};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let host: UpnpHost = Default::default();

    let overview = services::overview::overview(&host).await?;

    println!("{:#?}", overview);

    Ok(())
}
