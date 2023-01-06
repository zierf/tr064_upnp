use crate::{
    request_helper::{get_soap_action, UpnpHost},
    xml_nodes,
};

use serde_xml_rs::from_str;

xml_nodes! {
    struct Envelope {
        body: Body,
    }

    struct Body {
        get_addon_infos_response: GetAddonInfosResponse,
    }

    pub struct GetAddonInfosResponse {
        new_byte_send_rate: u32,
        new_byte_receive_rate: u32,
        new_packet_send_rate: u32,
        new_packet_receive_rate: u32,
        new_total_bytes_sent: u32,
        new_total_bytes_received: u32,
        new_auto_disconnect_time: u32,
        new_idle_disconnect_time: u32,
        #[serde(rename = "NewDNSServer1")]
        new_dns_server_1: String,
        #[serde(rename = "NewDNSServer2")]
        new_dns_server_2: String,
        #[serde(rename = "NewVoipDNSServer1")]
        new_voip_dns_server_1: String,
        #[serde(rename = "NewVoipDNSServer2")]
        new_voip_dns_server_2: String,
        new_upnp_control_enabled: bool,
        new_routed_bridged_mode_both: u8,
        #[serde(rename = "NewX_AVM_DE_TotalBytesSent64")]
        newx_avm_de_total_bytes_sent_64: String,
        #[serde(rename = "NewX_AVM_DE_TotalBytesReceived64")]
        newx_avm_de_total_bytes_received_64: String,
        #[serde(rename = "NewX_AVM_DE_WANAccessType")]
        newx_avm_de_wan_access_type: XAvmDeWanAccessType,
    }

    pub enum XAvmDeWanAccessType {
        #[serde(rename = "DSL")]
        Dsl,
        #[serde(rename = "ATA")]
        Ata,
        #[serde(rename = "USB-Tethering")]
        UsbTethering,
        Cable,
        #[serde(rename = "LTE")]
        Lte,
        Serial,
        #[serde(rename = "WLAN")]
        Wlan,
        #[serde(rename = "ISDN")]
        Isdn,
        #[serde(rename = "IP-Client")]
        IpClient,
        Fiber,
        Other,
    }
}

pub async fn get_addon_infos(host: UpnpHost) -> Result<GetAddonInfosResponse, reqwest::Error> {
    let result = get_soap_action(
        host,
        "/igdupnp/control/WANCommonIFC1",
        "WANCommonInterfaceConfig:1",
        "GetAddonInfos",
    )
    .await;

    let xml_string = result?;

    let addon_infos: Envelope = from_str(&xml_string).unwrap();

    Ok(addon_infos.body.get_addon_infos_response)
}
