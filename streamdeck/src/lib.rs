//! Stream Deck firmware library for Rust.

pub mod device;
mod error;
mod firmware;
mod device_lookup;

pub use device::{Device, StreamDeckMini};
pub use error::Error;
pub use firmware::{Firmware, FirmwareV1, FirmwareV2};