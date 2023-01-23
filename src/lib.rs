#![warn(missing_debug_implementations, rust_2018_idioms)] // TODO missing_docs

#[cfg(all(feature = "reqwest", feature = "esp32"))]
compile_error!("Feature \"reqwest\" and \"esp32\" must NOT be enabled at same time!");

mod macros;

mod error;

pub mod gateway;
pub mod request;

pub use error::{Error, Result};
pub use gateway::{Gateway, GatewayBuilder, SearchOptions};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Scheme {
    #[default]
    HTTP,
    HTTPS,
}
