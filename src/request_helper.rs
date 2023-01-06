use std::ops::Add;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};

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
            // "fritz.box:49000".to_socket_addrs()
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

fn default_headers() -> HeaderMap {
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
    headers.insert("Host", HeaderValue::from_static("fritz.box:49000"));

    headers
}

pub(crate) async fn get_soap_action(
    host: UpnpHost,
    endpoint: impl Into<String>,
    service_name: impl Into<String>,
    action: impl Into<String>,
) -> Result<String, reqwest::Error> {
    let protocol = match host.schema {
        Schema::HTTP => "http://".to_owned(),
        Schema::HTTPS => "https://".to_owned(),
    };

    let port = host.port.to_string();
    let endpoint: String = endpoint.into();

    let service_name: String = service_name.into();
    let action: String = action.into();

    let soap_action: String = format!("urn:schemas-upnp-org:service:{}#{}", service_name, action,);

    let envelope = format!(
        r#"
            <?xml version="1.0"?>
            <soap:Envelope
                xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
                soap:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/"
            >
                <soap:Header></soap:Header>
                <soap:Body>
                    <u:{} xmlns:u="urn:schemas-upnp-org:service:{}"></u:{}>
                </soap:Body>
            </soap:Envelope>
        "#,
        action, service_name, action,
    );

    let url =
        String::with_capacity(protocol.len() + host.name.len() + 1 + port.len() + endpoint.len())
            .add(&protocol)
            .add(&host.name)
            .add(":")
            .add(&port)
            .add(&endpoint);

    let mut headers = default_headers();

    headers.insert(
        "SOAPACTION",
        HeaderValue::from_str(soap_action.as_str())
            .unwrap_or_else(|_value| HeaderValue::from_static("")),
    );

    let client = reqwest::Client::builder()
        .http1_title_case_headers()
        .build()?;

    let builder = client.post(url).headers(headers).body(envelope);

    let response = builder.send().await?;

    response.text().await
}
