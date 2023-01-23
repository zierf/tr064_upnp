use crate::{xml_nodes_pascal_case, Gateway, Result};

use serde_xml_rs::from_str;

xml_nodes_pascal_case! {
    struct Envelope {
        body: Body,
    }

    struct Body {
        get_addon_infos_response: GetAddonInfosResponse,
    }

    pub struct GetAddonInfosResponse {
        pub new_byte_send_rate: u32,
        pub new_byte_receive_rate: u32,
        pub new_packet_send_rate: u32,
        pub new_packet_receive_rate: u32,
        pub new_total_bytes_sent: u32,
        pub new_total_bytes_received: u32,
        pub new_auto_disconnect_time: u32,
        pub new_idle_disconnect_time: u32,
        #[serde(rename = "NewDNSServer1")]
        pub new_dns_server_1: String,
        #[serde(rename = "NewDNSServer2")]
        pub new_dns_server_2: String,
        #[serde(rename = "NewVoipDNSServer1")]
        pub new_voip_dns_server_1: String,
        #[serde(rename = "NewVoipDNSServer2")]
        pub new_voip_dns_server_2: String,
        pub new_upnp_control_enabled: bool,
        pub new_routed_bridged_mode_both: u8,
        #[serde(rename = "NewX_AVM_DE_TotalBytesSent64")]
        pub newx_avm_de_total_bytes_sent_64: String,
        #[serde(rename = "NewX_AVM_DE_TotalBytesReceived64")]
        pub newx_avm_de_total_bytes_received_64: String,
        #[serde(rename = "NewX_AVM_DE_WANAccessType")]
        pub newx_avm_de_wan_access_type: XAvmDeWanAccessType,
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

impl Gateway {
    pub fn get_addon_infos(&self, version: Option<usize>) -> Result<GetAddonInfosResponse> {
        let xml_string = self.send_action(
            &format!("/igdupnp/control/WANCommonIFC{}", version.unwrap_or(1)),
            &format!(
                "urn:schemas-upnp-org:service:WANCommonInterfaceConfig:{}",
                version.unwrap_or(1)
            ),
            "GetAddonInfos",
        )?;

        let addon_infos: Envelope = from_str(&xml_string)?;

        Ok(addon_infos.body.get_addon_infos_response)
    }
}
