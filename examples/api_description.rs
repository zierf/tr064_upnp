use fritz_tr064_upnp::{services, UpnpHost};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host: UpnpHost = Default::default();

    let response = services::get_api_description(&host).await?;

    println!("{:#?}", response);

    Ok(())
}
