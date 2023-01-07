use fritz_tr064_upnp::{services, UpnpHost};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host: UpnpHost = Default::default();

    let response = services::get_service_description(&host, "/igdicfgSCPD.xml").await?;

    println!("{:#?}", response);

    Ok(())
}
