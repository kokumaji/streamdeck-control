//! Device-related functionality.

use hidapi::HidDevice;

use crate::{
    error::Error,
    firmware::{Firmware, FirmwareV1},
};

/// A trait representing a device implementing a particular firmware.
pub trait Device<F>
where
    F: Firmware,
{
    /// Returns the inner HID device.
    fn get_inner(&self) -> &HidDevice;

    /// Get the firmware version of the device.
    fn get_firmware_version(&self) -> Result<String, Error> {
        F::get_firmware_version(self.get_inner())
    }
}

/// A Stream Deck Mini device.
pub struct StreamDeckMini(HidDevice);

impl Device<FirmwareV1> for StreamDeckMini {
    fn get_inner(&self) -> &HidDevice {
        return &self.0;
    }
}
