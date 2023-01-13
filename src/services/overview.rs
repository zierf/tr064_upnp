use std::ops::RangeInclusive;

use async_recursion::async_recursion;

use crate::{error::Result, overview_json, request_helper::UpnpHost};

use super::description::{
    get_api_description, get_service_description, ArgumentDirection, StateVariable,
};

overview_json! {
    pub struct Device {
        pub name: String,
        pub udn: String,
        pub r#type: String,
        pub model_name: String,
        pub model_number: String,
        pub model_description: String,
        pub url: Option<String>,
        pub services: Vec<self::Service>,
        pub devices: Vec<Self>,
    }

    pub struct Service {
        pub id: String,
        pub r#type: String,
        pub control_url: String,
        pub scpd_url: String,
        pub actions: Vec<self::Action>,
    }

    pub struct Action {
        pub name: String,
        pub control_url: String,
        pub soap_action: String,
        pub inputs: Vec<Argument>,
        pub outputs: Vec<Argument>,
    }

    pub struct Argument {
        pub name: String,
        pub r#type: ArgumentType,
    }

    pub struct ArgumentType {
        pub name: String,
        pub r#type: Option<String>,
        pub default: Option<String>,
        pub allowed_values: Option<Vec<String>>,
        pub allowed_range: Option<AllowedRange>,
    }

    pub struct AllowedRange {
        pub range: RangeInclusive<usize>,
        pub step: Option<usize>,
    }
}

pub async fn overview(host: &UpnpHost) -> Result<self::Device> {
    let api_description = get_api_description(host).await?;

    let device = create_overview_for_device(host, &api_description.device).await?;

    Ok(device)
}

#[async_recursion]
async fn create_overview_for_device(
    host: &UpnpHost,
    device: &super::description::Device,
) -> Result<self::Device> {
    let services: Vec<self::Service> = futures::future::try_join_all(
        device
            .service_list
            .clone()
            .unwrap_or_else(|| super::description::ServiceList {
                services: Vec::new(),
            })
            .services
            .iter()
            .map(|service_entry| async move { map_service_from_api(host, service_entry).await }),
    )
    .await?;

    let devices: Vec<self::Device> = futures::future::try_join_all(
        device
            .device_list
            .clone()
            .unwrap_or_else(|| super::description::DeviceList {
                devices: Vec::new(),
            })
            .devices
            .iter()
            .map(|sub_device| async move { create_overview_for_device(host, sub_device).await }),
    )
    .await?;

    let device_overview = self::Device {
        name: device.friendly_name.clone(),
        udn: device.udn.clone(),
        r#type: device.device_type.clone(),
        model_name: device.model_name.clone(),
        model_number: device.model_number.clone(),
        model_description: device.model_description.clone(),
        url: device.presentation_url.clone(),
        services,
        devices,
    };

    Ok(device_overview)
}

async fn map_service_from_api(
    host: &UpnpHost,
    service: &super::description::Service,
) -> Result<self::Service> {
    let service_description = get_service_description(host, &service.scpd_url).await?;

    let argument_types: Vec<StateVariable> = service_description
        .service_state_table
        .state_variables
        .unwrap_or_default();

    let actions: Vec<self::Action> = service_description
        .action_list
        .actions
        .unwrap_or_default()
        .iter()
        .map(|desc_action| {
            let argument_list = extract_arguments_from_action(desc_action);

            Action {
                name: desc_action.name.clone(),
                control_url: service.control_url.clone(),
                soap_action: format!("{}#{}", service.service_type, desc_action.name),
                inputs: argument_list
                    .iter()
                    .filter(|argument| argument.direction == ArgumentDirection::In)
                    .map(|argument| combine_argument_and_type(argument, &argument_types))
                    .collect(),
                outputs: argument_list
                    .iter()
                    .filter(|argument| argument.direction == ArgumentDirection::Out)
                    .map(|argument| combine_argument_and_type(argument, &argument_types))
                    .collect(),
            }
        })
        .collect();

    let service = Service {
        id: service.service_id.clone(),
        r#type: service.service_type.clone(),
        control_url: service.control_url.clone(),
        scpd_url: service.scpd_url.clone(),
        actions,
    };

    Ok(service)
}

fn extract_arguments_from_action(
    action: &super::description::Action,
) -> Vec<super::description::Argument> {
    action
        .argument_list
        .clone()
        .unwrap_or_else(|| super::description::ArgumentList {
            arguments: Vec::new(),
        })
        .arguments
}

fn combine_argument_and_type(
    argument: &super::description::Argument,
    argument_types: &[StateVariable],
) -> self::Argument {
    let arg_type: Option<&StateVariable> = argument_types
        .iter()
        .find(|argt| argt.name == argument.r#type);

    Argument {
        name: argument.name.clone(),
        r#type: ArgumentType {
            name: argument.r#type.clone(),
            r#type: arg_type.map(|argt| argt.r#type.clone()),
            default: arg_type.and_then(|argt| argt.default_value.clone()),
            allowed_values: arg_type
                .and_then(|argt| argt.allowed_value_list.clone())
                .map(|list| list.allowed_values),
            allowed_range: arg_type
                .and_then(|argt| argt.allowed_range.clone())
                .map(|range| AllowedRange {
                    range: range.minimum..=range.maximum,
                    step: range.step,
                }),
        },
    }
}
