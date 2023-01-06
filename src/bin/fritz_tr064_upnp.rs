use fritz_tr064_upnp::{get_addon_infos, UpnpHost};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host: UpnpHost = Default::default();

    let resp = get_addon_infos(host);

    println!("{:?}", resp.await);

    Ok(())
}
