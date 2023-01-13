use std::ops::Add;

use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT},
    Client, Response,
};

use crate::error::{Error, Result};

pub static DEFAULT_HOST: &'static str = "fritz.box:49000";

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Schema {
    #[default]
    HTTP,
    HTTPS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpnpHost {
    name: String,
    port: u16,
    schema: Schema,
}

impl Default for UpnpHost {
    fn default() -> Self {
        Self {
            // DEFAULT_HOST.to_socket_addrs()
            name: "fritz.box".to_owned(),
            port: 49000,
            schema: Schema::HTTP,
        }
    }
}

impl UpnpHost {
    pub fn builder() -> UpnpHostBuilder {
        UpnpHostBuilder::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct UpnpHostBuilder {
    host: UpnpHost,
}

impl UpnpHostBuilder {
    pub fn new() -> Self {
        let builder: Self = Default::default();
        builder
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.host.name = name.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.host.port = port;
        self
    }

    pub fn schema(mut self, schema: Schema) -> Self {
        self.host.schema = schema;
        self
    }

    pub fn build(&self) -> UpnpHost {
        self.host.clone()
    }
}

pub(crate) async fn get_api_xml(host: &UpnpHost, endpoint: &str) -> Result<Response> {
    let url = endpoint_url(host, endpoint);

    let client = Client::builder().http1_title_case_headers().build()?;

    let builder = client.get(url).headers(default_headers(host));

    let response = builder.send().await?;

    if !response.status().is_success() {
        return Err(Error::StatusCodeError {
            status_code: response.status().as_u16(),
            message: response.text().await?,
        });
    }

    Ok(response)
}

pub async fn send_soap_action(
    host: &UpnpHost,
    endpoint: &str,
    service_type: &str,
    action: &str,
) -> Result<Response> {
    let soap_action: String = format!("{service_type}#{action}");

    let mut headers = default_headers(host);

    headers.insert(
        "SOAPACTION",
        HeaderValue::from_str(soap_action.as_str())
            .unwrap_or_else(|_value| HeaderValue::from_static("")),
    );

    let url = endpoint_url(host, endpoint);

    let envelope = build_envelope(service_type, action);

    let client = Client::builder().http1_title_case_headers().build()?;

    let builder = client.post(url).headers(headers).body(envelope);

    let response = builder.send().await?;

    if !response.status().is_success() {
        return Err(Error::StatusCodeError {
            status_code: response.status().as_u16(),
            message: response.text().await?,
        });
    }

    Ok(response)
}

pub fn default_headers(host: &UpnpHost) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("AVM UPnP/1.0 Client 1.0"),
    );
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/xml; charset=\"utf-8\" User-Agent: AVM UPnP/1.0 Client 1.0"),
    );

    // allow case sensitive headers (also necessary for `SOAPACTION`).
    // https://github.com/seanmonstar/reqwest/pull/463#issue-414510806
    let host_address = format!("{}:{}", host.name, host.port);
    headers.insert(
        "Host",
        HeaderValue::from_str(&host_address)
            .unwrap_or_else(|_val| HeaderValue::from_static(DEFAULT_HOST)),
    );

    headers
}

pub fn endpoint_url(host: &UpnpHost, endpoint: &str) -> String {
    let protocol = match host.schema {
        Schema::HTTP => "http://".to_owned(),
        Schema::HTTPS => "https://".to_owned(),
    };

    let port = host.port.to_string();

    String::with_capacity(protocol.len() + host.name.len() + 1 + port.len() + endpoint.len())
        .add(&protocol)
        .add(&host.name)
        .add(":")
        .add(&port)
        .add(endpoint)
}

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
