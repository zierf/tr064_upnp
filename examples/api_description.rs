use anyhow::Result;

use fritz_tr064_upnp::{services::description::get_api_description, UpnpHost};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let host: UpnpHost = Default::default();

    let response = get_api_description(&host).await?;

    println!("{:#?}", response);

    Ok(())
}
