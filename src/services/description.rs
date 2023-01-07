use crate::{
    request_helper::{get_api_xml, UpnpHost},
    xml_nodes_camel_case,
};

use serde_xml_rs::from_str;

xml_nodes_camel_case! {
    pub struct SpecVersion {
        major: usize,
        minor: usize,
    }
}

xml_nodes_camel_case! {
    pub struct ApiDescription {
        spec_version: SpecVersion,
        device: Device,
    }

    pub struct Device {
        device_type: String,
        friendly_name: String,
        manufacturer: String,
        #[serde(rename = "manufacturerURL")]
        manufacturer_url: String,
        model_description: String,
        model_name: String,
        model_number: String,
        #[serde(rename = "modelURL")]
        model_url: String,
        #[serde(rename = "UDN")]
        udn: String,
        #[serde(rename = "UPC")]
        upc: Option<String>,
        icon_list: Option<IconList>,
        service_list: Option<ServiceList>,
        device_list: Option<DeviceList>,
        #[serde(rename = "presentationURL")]
        presentation_url: Option<String>,
    }

    pub struct IconList {
        #[serde(rename = "$value")]
        icons: Vec<Icon>,
    }

    pub struct Icon {
        mimetype: String,
        width: usize,
        height: usize,
        depth: usize,
        url: String,
    }

    pub struct ServiceList {
        #[serde(rename = "$value")]
        services: Vec<Service>,
    }

    pub struct Service {
        service_type: String,
        service_id: String,
        #[serde(rename = "controlURL")]
        control_url: String,
        #[serde(rename = "eventSubURL")]
        event_sub_url: String,
        #[serde(rename = "SCPDURL")]
        scpdurl: String,
    }

    pub struct DeviceList {
        #[serde(rename = "$value")]
        devices: Vec<Device>,
    }
}

xml_nodes_camel_case! {
    pub struct ServiceDescription {
        spec_version: SpecVersion,
        action_list: ActionList,
        service_state_table: ServiceStateTable,
    }

    pub struct ActionList {
        #[serde(rename = "$value")]
        actions: Vec<Action>,
    }

    pub struct Action {
        name: String,
        #[serde(rename = "argumentList")]
        arguments: ArgumentList,
    }

    pub struct ArgumentList {
        #[serde(rename = "$value")]
        arguments: Vec<Argument>,
    }

    pub struct Argument {
        name: String,
        #[serde(rename = "relatedStateVariable")]
        r#type: String,
        direction: ArgumentDirection,
    }

    pub enum ArgumentDirection {
        In,
        Out,
    }

    pub struct ServiceStateTable {
        #[serde(rename = "$value")]
        state_variables: Vec<StateVariable>,
    }

    pub struct StateVariable {
        name: String,
        #[serde(rename = "dataType")]
        r#type: String,
        default_value: Option<String>,
        #[serde(rename = "allowedValueList")]
        allowed_values: Option<AllowedValueList>,
        #[serde(rename = "allowedValueRange")]
        allowed_range: Option<AllowedValueRange>,
    }

    pub struct AllowedValueList {
        #[serde(rename = "$value")]
        allowed_values: Vec<String>,
    }

    pub struct AllowedValueRange {
        minimum: usize,
        maximum: usize,
        step: usize,
    }
}

pub async fn get_api_description(host: &UpnpHost) -> Result<ApiDescription, reqwest::Error> {
    let result = get_api_xml(host, "/igddesc.xml").await;

    let xml_string = result?;

    let service_description: ApiDescription = from_str(&xml_string).unwrap();

    Ok(service_description)
}

pub async fn get_service_description(
    host: &UpnpHost,
    endpoint: &str,
) -> Result<ServiceDescription, reqwest::Error> {
    let result = get_api_xml(host, endpoint).await;

    let xml_string = result?;

    let service_description: ServiceDescription = from_str(&xml_string).unwrap();

    Ok(service_description)
}
