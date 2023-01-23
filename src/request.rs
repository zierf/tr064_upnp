use std::ops::Add;

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT},
};

use crate::{gateway::DEFAULT_HOSTNAME, Error, Gateway, Result, Scheme};

pub(crate) fn get_api_xml(gateway: &Gateway, endpoint: &str) -> Result<Response> {
    let url = gateway.endpoint_url(endpoint);

    let client = Client::builder().http1_title_case_headers().build()?;

    let builder = client.get(url).headers(gateway.default_headers());

    let response = builder.send()?;

    if !response.status().is_success() {
        return Err(Error::StatusCodeError {
            status_code: response.status().as_u16(),
            message: response.text()?,
        });
    }

    Ok(response)
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

impl Gateway {
    pub fn send_action(
        &self,
        endpoint: &str,
        service_type: &str,
        action: &str,
    ) -> Result<Response> {
        let soap_action: String = format!("{service_type}#{action}");

        let mut headers = self.default_headers();

        headers.insert(
            "SOAPACTION",
            HeaderValue::from_str(soap_action.as_str())
                .unwrap_or_else(|_value| HeaderValue::from_static("")),
        );

        let url = self.endpoint_url(endpoint);

        let envelope = build_envelope(service_type, action);

        let client = Client::builder().http1_title_case_headers().build()?;

        let builder = client.post(url).headers(headers).body(envelope);

        let response = builder.send()?;

        if !response.status().is_success() {
            return Err(Error::StatusCodeError {
                status_code: response.status().as_u16(),
                message: response.text()?,
            });
        }

        Ok(response)
    }

    pub fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("AVM UPnP/1.0 Client 1.0"),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static(
                "text/xml; charset=\"utf-8\" User-Agent: AVM UPnP/1.0 Client 1.0",
            ),
        );

        // allow case sensitive headers (also necessary for `SOAPACTION`).
        // https://github.com/seanmonstar/reqwest/pull/463#issue-414510806
        let gateway_address = format!("{}:{}", self.host, self.port);

        headers.insert(
            "Host",
            HeaderValue::from_str(&gateway_address)
                .unwrap_or_else(|_val| HeaderValue::from_static(DEFAULT_HOSTNAME)),
        );

        headers
    }

    pub fn endpoint_url(&self, endpoint: &str) -> String {
        let protocol = match self.scheme {
            Scheme::HTTP => "http://".to_owned(),
            Scheme::HTTPS => "https://".to_owned(),
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
