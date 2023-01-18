mod overview;
pub mod services;

use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::str;
use std::time::Duration;

use url::{Host, Url};

use crate::{Error, Result, Scheme};

pub static BROADCAST_IPV4: &str = "239.255.255.250:1900";
pub static BROADCAST_IPV6: &str = "[FF02::C]:1900";

pub(crate) static DEFAULT_HOSTNAME: &str = "fritz.box";
pub(crate) static DEFAULT_HOSTPORT: u16 = 49000;

#[derive(Clone, Debug, PartialEq)]
pub struct Gateway {
    pub host: Host<String>,
    pub port: u16,
    pub scheme: Scheme,
    pub root_url: Option<String>,
}

impl Default for Gateway {
    fn default() -> Self {
        // defaults for a Fritz!Box
        Self {
            host: Host::parse(DEFAULT_HOSTNAME).expect("Default host should be parsable."),
            port: DEFAULT_HOSTPORT,
            scheme: Scheme::HTTP,
            root_url: Some("/igddesc.xml".to_owned()),
        }
    }
}

impl Gateway {
    pub fn builder() -> GatewayBuilder {
        GatewayBuilder::default()
    }

    pub async fn discover(options: SearchOptions) -> Result<Gateway> {
        let socket = UdpSocket::bind(options.bind_address)?;
        socket.set_read_timeout(options.timeout)?;

        // ST: ssdp:all
        let request = format!(
            r#"M-SEARCH * HTTP/1.1
Host: {}
Man: "ssdp:discover"
ST: urn:schemas-upnp-org:device:InternetGatewayDevice:1
MX: 2

"#,
            options.broadcast_address
        );

        socket.send_to(request.as_bytes(), options.broadcast_address)?;

        let mut buf = [0u8; 1500];
        let (read, _) = socket.recv_from(&mut buf)?;
        let text = str::from_utf8(&buf[..read])?;

        let (scheme, host, port, root_url) = parse_search_response(text)?;

        Ok(Gateway {
            host,
            port,
            scheme,
            root_url: Some(root_url),
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct GatewayBuilder {
    gateway: Gateway,
}

impl GatewayBuilder {
    pub fn new() -> Self {
        let builder: Self = Default::default();
        builder
    }

    pub fn host(mut self, host: impl Into<Host>) -> Self {
        self.gateway.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.gateway.port = port;
        self
    }

    pub fn scheme(mut self, scheme: Scheme) -> Self {
        self.gateway.scheme = scheme;
        self
    }

    pub fn root_url(mut self, root_url: Option<String>) -> Self {
        self.gateway.root_url = root_url;
        self
    }

    pub fn build(&self) -> Gateway {
        self.gateway.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchOptions {
    /// Bind address for UDP socket, defaults to all `0.0.0.0`.
    pub bind_address: SocketAddr,

    /// Broadcast address for discovery packets.
    ///
    /// Defaults to IPv4 `239.255.255.250:1900`.
    /// For IPv6 use `[FF02::C]:1900` as broadcast address.
    pub broadcast_address: SocketAddr,

    /// Timeout for a search iteration (defaults to 5s).
    pub timeout: Option<Duration>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            bind_address: SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), 0),
            broadcast_address: SocketAddr::new(Ipv4Addr::new(239, 255, 255, 250).into(), 1900),
            timeout: Some(Duration::from_secs(5)),
        }
    }
}

fn parse_search_response(message: &str) -> Result<(Scheme, Host<String>, u16, String)> {
    for line in message.lines() {
        let line = line.trim();

        if line.to_ascii_lowercase().starts_with("location:") {
            if let Some(colon) = line.find(':') {
                let url_text = line[colon + 1..].trim();
                let url = Url::parse(url_text).map_err(|_| Error::SearchError(message.into()))?;

                let scheme = if url.scheme() == "http" {
                    Scheme::HTTP
                } else {
                    Scheme::HTTPS
                };

                let host: Host<String> = url
                    .host()
                    .map(|value| value.to_owned())
                    .ok_or(Error::SearchError(message.into()))?;

                let port: u16 = url
                    .port_or_known_default()
                    .ok_or(Error::SearchError(message.into()))?;

                return Ok((scheme, host, port, url.path().to_owned()));
            }
        }
    }

    Err(Error::SearchError(message.into()))
}
