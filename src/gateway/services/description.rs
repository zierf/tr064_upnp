use crate::{request::get_api_xml, xml_nodes_camel_case, Gateway, Result};

use serde_xml_rs::from_str;

xml_nodes_camel_case! {
    pub struct SpecVersion {
        pub major: usize,
        pub minor: usize,
    }
}

xml_nodes_camel_case! {
    pub struct ApiDescription {
        pub spec_version: SpecVersion,
        #[serde(rename = "URLBase")]
        pub url_base: Option<String>,
        pub device: Device,
    }

    pub struct Device {
        pub device_type: String,
        pub friendly_name: String,
        pub manufacturer: String,
        #[serde(rename = "manufacturerURL")]
        pub manufacturer_url: String,
        pub model_description: String,
        pub model_name: String,
        pub model_number: String,
        #[serde(rename = "modelURL")]
        pub model_url: String,
        #[serde(rename = "UDN")]
        pub udn: String,
        #[serde(rename = "UPC")]
        pub upc: Option<String>,
        pub icon_list: Option<IconList>,
        pub service_list: Option<ServiceList>,
        pub device_list: Option<DeviceList>,
        #[serde(rename = "presentationURL")]
        pub presentation_url: Option<String>,
    }

    pub struct IconList {
        #[serde(rename = "$value")]
        pub icons: Vec<Icon>,
    }

    pub struct Icon {
        pub mimetype: String,
        pub width: usize,
        pub height: usize,
        pub depth: usize,
        pub url: String,
    }

    pub struct ServiceList {
        #[serde(rename = "$value")]
        pub services: Vec<Service>,
    }

    pub struct Service {
        pub service_type: String,
        pub service_id: String,
        #[serde(rename = "controlURL")]
        pub control_url: String,
        #[serde(rename = "eventSubURL")]
        pub event_sub_url: String,
        #[serde(rename = "SCPDURL")]
        pub scpd_url: String,
    }

    pub struct DeviceList {
        #[serde(rename = "$value")]
        pub devices: Vec<Device>,
    }
}

xml_nodes_camel_case! {
    pub struct ServiceDescription {
        pub spec_version: SpecVersion,
        pub action_list: ActionList,
        pub service_state_table: ServiceStateTable,
    }

    pub struct ActionList {
        #[serde(rename = "$value")]
        pub actions: Option<Vec<Action>>,
    }

    pub struct Action {
        pub name: String,
        #[serde(rename = "argumentList")]
        pub argument_list: Option<ArgumentList>,
    }

    pub struct ArgumentList {
        #[serde(rename = "$value")]
        pub arguments: Vec<Argument>,
    }

    pub struct Argument {
        pub name: String,
        #[serde(rename = "relatedStateVariable")]
        pub r#type: String,
        pub direction: ArgumentDirection,
    }

    pub enum ArgumentDirection {
        In,
        Out,
    }

    pub struct ServiceStateTable {
        #[serde(rename = "$value")]
        pub state_variables: Option<Vec<StateVariable>>,
    }

    pub struct StateVariable {
        pub name: String,
        #[serde(rename = "dataType")]
        pub r#type: String,
        pub default_value: Option<String>,
        #[serde(rename = "allowedValueList")]
        pub allowed_value_list: Option<AllowedValueList>,
        #[serde(rename = "allowedValueRange")]
        pub allowed_range: Option<AllowedValueRange>,
    }

    pub struct AllowedValueList {
        #[serde(rename = "$value")]
        pub allowed_values: Vec<String>,
    }

    pub struct AllowedValueRange {
        pub minimum: usize,
        pub maximum: usize,
        pub step: Option<usize>,
    }
}

impl Gateway {
    pub async fn api_description(&self) -> Result<ApiDescription> {
        let response = get_api_xml(self, "/igddesc.xml").await?;

        let xml_string = response.text().await?;

        let service_description: ApiDescription = from_str(&xml_string)?;

        Ok(service_description)
    }

    pub async fn service_description(&self, endpoint: &str) -> Result<ServiceDescription> {
        let response = get_api_xml(self, endpoint).await?;

        let xml_string = response.text().await?;

        let service_description: ServiceDescription = from_str(&xml_string)?;

        Ok(service_description)
    }
}
