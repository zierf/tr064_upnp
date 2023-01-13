use anyhow::Result;

use fritz_tr064_upnp::{services::*, Schema, UpnpHost, UpnpHostBuilder};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let host: UpnpHost = UpnpHostBuilder::new()
        .name("fritz.box")
        .port(49000)
        .schema(Schema::HTTP)
        .build();

    let response = wan_common_interface_config::get_addon_infos(&host, None).await?;

    println!("{:#?}", response);

    Ok(())
}
