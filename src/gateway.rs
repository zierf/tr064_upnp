mod overview;
pub mod services;

use std::marker::PhantomData;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::str;
use std::time::Duration;

use url::{Host, Url};

use crate::{Error, Result, Scheme};

pub static BROADCAST_IPV4: &str = "239.255.255.250:1900";
pub static BROADCAST_IPV6: &str = "[FF02::C]:1900";

pub static DEFAULT_HOSTNAME: &str = "fritz.box";
pub static DEFAULT_HOSTPORT: u16 = 49000;
pub static DEFAULT_ROOT_URL: &str = "/igddesc.xml";

#[derive(Clone, Debug, PartialEq)]
pub struct Gateway {
    pub host: Host<String>,
    pub port: u16,
    pub scheme: Scheme,
    pub root_url: String,
}

impl Default for Gateway {
    fn default() -> Self {
        // defaults for a Fritz!Box
        Self {
            host: Host::parse(DEFAULT_HOSTNAME).expect("Default host should be parsable as host."),
            port: DEFAULT_HOSTPORT,
            scheme: Scheme::HTTP,
            root_url: DEFAULT_ROOT_URL.to_owned(),
        }
    }
}

impl Gateway {
    /// Create a new gateway builder.
    pub fn builder() -> GatewayBuilder<NoHost, NoRootUrl, NotSealed> {
        GatewayBuilder::new()
    }

    pub fn discover(options: SearchOptions) -> Result<Gateway> {
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
            root_url,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct GatewayBuilder<H, U, S> {
    host: H,
    port: u16,
    scheme: Scheme,
    root_url: U,
    marker_seal: PhantomData<S>,
}

// region:    --- GatewayBuilder States
#[derive(Clone, Debug, Default)]
pub struct Sealed;
#[derive(Clone, Debug, Default)]
pub struct NotSealed;

#[derive(Clone, Debug, Default)]
pub struct NoHost;
#[derive(Clone, Debug, Default)]
pub struct WithHost(String); // Host<String>

#[derive(Clone, Debug, Default)]
pub struct NoRootUrl;
#[derive(Clone, Debug, Default)]
pub struct WithRootUrl(String);
// endregion: --- GatewayBuilder States

impl GatewayBuilder<NoHost, NoRootUrl, NotSealed> {
    /// Create a new gateway builder.
    pub fn new() -> Self {
        Self {
            host: NoHost,
            port: DEFAULT_HOSTPORT,
            scheme: Scheme::HTTP,
            root_url: NoRootUrl,
            marker_seal: PhantomData,
        }
    }
}

impl<U, M> GatewayBuilder<U, M, NotSealed> {
    /// Seal builder and prevent further modifications.
    ///
    /// Builder can still be cloned afterwards to call the `build` method multiple times.
    pub fn seal(self) -> GatewayBuilder<U, M, Sealed> {
        GatewayBuilder {
            host: self.host,
            port: self.port,
            scheme: self.scheme,
            root_url: self.root_url,
            marker_seal: PhantomData,
        }
    }
}

impl<S> GatewayBuilder<WithHost, WithRootUrl, S> {
    /// Create a gateway with given builder options.
    pub fn build(self) -> Result<Gateway> {
        let gateway = Gateway {
            host: Host::parse(&self.host.0)?,
            port: self.port,
            scheme: self.scheme,
            root_url: self.root_url.0,
        };

        Ok(gateway)
    }
}

impl<H, U> GatewayBuilder<H, U, NotSealed> {
    /// Gateway hostname (`fritz.box`), IPv4 address (`192.168.178.1`) or IPv6 address (`[1234:5678:90ab::10a]`).
    pub fn host(self, host: impl Into<String>) -> GatewayBuilder<WithHost, U, NotSealed> {
        GatewayBuilder {
            host: WithHost(host.into()),
            port: self.port,
            scheme: self.scheme,
            root_url: self.root_url,
            marker_seal: PhantomData,
        }
    }

    /// Gateway UPnP port.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Gateway UPnP protocol scheme.
    pub fn scheme(mut self, scheme: Scheme) -> Self {
        self.scheme = scheme;
        self
    }

    /// Location to XML with api description.
    pub fn root_url(
        self,
        root_url: impl Into<String>,
    ) -> GatewayBuilder<H, WithRootUrl, NotSealed> {
        GatewayBuilder {
            host: self.host,
            port: self.port,
            scheme: self.scheme,
            root_url: WithRootUrl(root_url.into()),
            marker_seal: PhantomData,
        }
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
