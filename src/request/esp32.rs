use embedded_svc::{
    http::{client::Client, Method, Status},
    io::Write,
    utils::io,
};
use esp_idf_svc::http::client::{Configuration as HttpConfiguration, EspHttpConnection};

use crate::{Error, Gateway, Result};

const BUFFER_SIZE: usize = 16;

// #[derive(Clone)]
// struct HeaderTuple<'a>(&'a str, &'a str);

/// Create HTTP(S) client
fn create_client() -> Result<Client<EspHttpConnection>> {
    let client = Client::wrap(EspHttpConnection::new(&HttpConfiguration {
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach), // Needed for HTTPS support
        ..Default::default()
    })?);

    Ok(client)
}

pub fn get_api_xml(gateway: &Gateway, endpoint: &str) -> Result<String> {
    let gateway_address = format!("{}:{}", gateway.host, gateway.port);

    // TODO use gateway.default_headers()
    let headers = [
        ("User-Agent", "AVM UPnP/1.0 Client 1.0"),
        (
            "Content-Type",
            "text/xml; charset=\"utf-8\" User-Agent: AVM UPnP/1.0 Client 1.0",
        ),
        ("Host", &gateway_address),
        ("accept", "text/xml"),
        ("connection", "close"),
    ];

    let url = gateway.endpoint_url(endpoint);

    let mut client = create_client()?;

    let request = client.request(Method::Get, &url, &headers)?;

    let mut response = request.submit()?;

    // Process response
    let status = response.status();

    let (_headers, mut body) = response.split();

    // TODO find a cleaner way to read the body
    let mut buf = [0u8; BUFFER_SIZE];
    let mut byte_vec: Vec<u8> = Vec::with_capacity(0);

    loop {
        let bytes_read = io::try_read_full(&mut body, &mut buf).map_err(|e| e.0)?;

        if bytes_read == 0 {
            break;
        }

        byte_vec.extend_from_slice(&buf[..bytes_read]);
    }

    let response_body = String::from_utf8(byte_vec)?;

    if status != 200u16 {
        return Err(Error::StatusCodeError {
            status_code: status,
            message: response_body,
        });
    }

    Ok(response_body)
}

impl Gateway {
    pub fn send_action(&self, endpoint: &str, service_type: &str, action: &str) -> Result<String> {
        let gateway_address = format!("{}:{}", self.host, self.port);
        let soap_action: String = format!("{service_type}#{action}");

        // TODO use gateway.default_headers()
        let headers = [
            ("User-Agent", "AVM UPnP/1.0 Client 1.0"),
            (
                "Content-Type",
                "text/xml; charset=\"utf-8\" User-Agent: AVM UPnP/1.0 Client 1.0",
            ),
            ("Host", &gateway_address),
            ("SOAPACTION", &soap_action),
            ("accept", "text/xml"),
            ("connection", "close"),
        ];

        let url = self.endpoint_url(endpoint);
        let envelope = super::build_envelope(service_type, action);

        let mut client = create_client()?;

        let mut request = client.post(&url, &headers)?;
        request.write_all(envelope.as_bytes())?;
        request.flush()?;

        let mut response = request.submit()?;

        // Process response
        let status = response.status();

        let (_headers, mut body) = response.split();

        // TODO find a cleaner way to read the body
        let mut buf = [0u8; BUFFER_SIZE];
        let mut byte_vec: Vec<u8> = Vec::with_capacity(0);

        loop {
            let bytes_read = io::try_read_full(&mut body, &mut buf).map_err(|e| e.0)?;

            if bytes_read == 0 {
                break;
            }

            byte_vec.extend_from_slice(&buf[..bytes_read]);
        }

        let response_body = String::from_utf8(byte_vec)?;

        if status != 200u16 {
            return Err(Error::StatusCodeError {
                status_code: status,
                message: response_body,
            });
        }

        Ok(response_body)
    }

    /*fn default_headers(&self) -> Vec<HeaderTuple<'_>> {
        let gateway_address = format!("{}:{}", self.host, self.port);

        let headers: Vec<HeaderTuple<'_>> = vec![
            HeaderTuple("User-Agent", "AVM UPnP/1.0 Client 1.0"),
            HeaderTuple(
                "Content-Type",
                "text/xml; charset=\"utf-8\" User-Agent: AVM UPnP/1.0 Client 1.0",
            ),
            HeaderTuple("Host", &gateway_address),
        ];

        headers
    }*/
}
