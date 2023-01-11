use fritz_tr064_upnp::{services, UpnpHost};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host: UpnpHost = Default::default();

    let response = services::overview::overview(&host).await?;

    println!("{:#?}", response);

    Ok(())
}
