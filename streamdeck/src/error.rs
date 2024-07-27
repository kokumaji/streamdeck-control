//! Error handling types.

use hidapi::HidError;
use thiserror::Error;

/// An enumeration of possible errors.
#[derive(Debug, Error)]
pub enum Error {
    /// A firmware-related error.
    #[error("HID error: {0}")]
    FirmwareError(#[from] HidError),
}
