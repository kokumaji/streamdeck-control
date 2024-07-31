//! Stream Deck firmware library for Rust.

mod device;
mod error;
mod firmware;
mod hid;

pub use device::{Connect, Device, StreamDeckMini};
pub use error::Error;
