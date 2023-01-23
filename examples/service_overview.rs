use anyhow::Result;

use fritz_tr064_upnp::{Gateway, SearchOptions};

fn main() -> Result<()> {
    let gateway = Gateway::discover(SearchOptions::default())?;

    let overview = gateway.overview()?;

    println!("{:#?}", overview);

    Ok(())
}
