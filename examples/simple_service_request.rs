use anyhow::Result;

use fritz_tr064_upnp::{Gateway, SearchOptions};

fn main() -> Result<()> {
    let gateway = Gateway::discover(SearchOptions::default())?;

    let response = gateway.get_addon_infos(None)?;

    println!("{:#?}", response);

    Ok(())
}
