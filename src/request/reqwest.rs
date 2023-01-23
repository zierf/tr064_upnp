use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT},
};

use crate::{gateway::DEFAULT_HOST_NAME, Error, Gateway, Result};

pub fn get_api_xml(gateway: &Gateway, endpoint: &str) -> Result<String> {
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

    Ok(response.text()?)
}

impl Gateway {
    pub fn send_action(&self, endpoint: &str, service_type: &str, action: &str) -> Result<String> {
        let soap_action: String = format!("{service_type}#{action}");

        let mut headers = self.default_headers();

        headers.insert(
            "SOAPACTION",
            HeaderValue::from_str(soap_action.as_str())
                .unwrap_or_else(|_value| HeaderValue::from_static("")),
        );

        let url = self.endpoint_url(endpoint);

        let envelope = super::build_envelope(service_type, action);

        let client = Client::builder().http1_title_case_headers().build()?;

        let builder = client.post(url).headers(headers).body(envelope);

        let response = builder.send()?;

        if !response.status().is_success() {
            return Err(Error::StatusCodeError {
                status_code: response.status().as_u16(),
                message: response.text()?,
            });
        }

        Ok(response.text()?)
    }

    fn default_headers(&self) -> HeaderMap {
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

        let gateway_address = format!("{}:{}", self.host, self.port);

        // allow case sensitive headers (also necessary for `SOAPACTION`).
        // https://github.com/seanmonstar/reqwest/pull/463#issue-414510806
        headers.insert(
            "Host",
            HeaderValue::from_str(&gateway_address)
                .unwrap_or_else(|_val| HeaderValue::from_static(DEFAULT_HOST_NAME)),
        );

        headers
    }
}
