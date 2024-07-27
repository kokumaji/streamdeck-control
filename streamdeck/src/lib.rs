//! Stream Deck firmware library for Rust.

mod device;
mod error;
mod firmware;

pub use device::{Device, StreamDeckMini};
pub use error::Error;
pub use firmware::{Firmware, FirmwareV1, FirmwareV2};
