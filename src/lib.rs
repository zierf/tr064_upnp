#![warn(missing_debug_implementations, rust_2018_idioms)] // TODO missing_docs

mod request_helper;

pub use request_helper::{get_soap_action, Schema, UpnpHost};

pub async fn get_addon_infos(host: UpnpHost) -> Result<String, reqwest::Error> {
    let result = get_soap_action(
        host,
        "/igdupnp/control/WANCommonIFC1",
        "WANCommonInterfaceConfig:1",
        "GetAddonInfos",
    )
    .await;

    result
}
