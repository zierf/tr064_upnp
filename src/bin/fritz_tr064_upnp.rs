use fritz_tr064_upnp::{services::*, UpnpHost};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host: UpnpHost = Default::default();

    let response = wan_common_interface_config::get_addon_infos(host).await?;

    println!("{:#?}", response);

    Ok(())
}
