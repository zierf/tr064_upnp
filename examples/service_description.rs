use fritz_tr064_upnp::{services::description::get_service_description, UpnpHost};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host: UpnpHost = Default::default();

    let dummy_description = get_service_description(&host, "/any.xml").await?;
    let response = get_service_description(&host, "/igdicfgSCPD.xml").await?;

    println!("{:#?}", dummy_description);
    println!("{:#?}", response);

    Ok(())
}
