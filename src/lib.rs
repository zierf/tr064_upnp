#![warn(missing_debug_implementations, rust_2018_idioms)] // TODO missing_docs

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
