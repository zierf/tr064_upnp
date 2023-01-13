#![warn(missing_debug_implementations, rust_2018_idioms)] // TODO missing_docs

mod macros;

pub mod error;
pub mod request_helper;
pub mod services;

pub use request_helper::{Schema, UpnpHost, UpnpHostBuilder};
