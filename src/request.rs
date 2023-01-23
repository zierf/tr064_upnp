#[cfg(target_os = "espidf")]
mod esp32;
#[cfg(not(target_os = "espidf"))]
mod reqwest;

#[cfg(target_os = "espidf")]
pub use self::esp32::*;
#[cfg(not(target_os = "espidf"))]
pub use self::reqwest::*;

use std::ops::Add;

use crate::{Gateway, Scheme};

pub fn build_envelope(service_type: &str, action_name: &str) -> String {
    format!(
        r#"
            <?xml version="1.0"?>
            <soap:Envelope
                xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
                soap:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/"
            >
                <soap:Header></soap:Header>
                <soap:Body>
                    <u:{action_name} xmlns:u="{service_type}"></u:{action_name}>
                </soap:Body>
            </soap:Envelope>
        "#
    )
}

impl Gateway {
    pub fn endpoint_url(&self, endpoint: &str) -> String {
        let protocol = match self.scheme {
            Scheme::HTTP => "http://".to_string(),
            Scheme::HTTPS => "https://".to_string(),
        };

        let host = self.host.to_string();
        let port = self.port.to_string();

        String::with_capacity(protocol.len() + host.len() + 1 + port.len() + endpoint.len())
            .add(&protocol)
            .add(&host)
            .add(":")
            .add(&port)
            .add(endpoint)
    }
}
