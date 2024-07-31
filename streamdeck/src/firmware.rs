//! Defines firmware traits and implementations for Stream Deck devices.

use hidapi::HidDevice;

use crate::error::Error;

pub const FW_BMP_HEADER: [u8; 54] = [
    0x42, 0x4d, 0xf6, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00,
    0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xc0, 0x3c, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// The firmware trait represents the firmware of a Stream Deck device.
pub trait Firmware {
    type FeatureReportBuffer: AsRef<[u8]>;
    type FeatureReportBufferMut: AsMut<[u8]>;

    /// The offset of the firmware version in the feature report.
    fn get_firmware_version_offset() -> usize;

    /// Get the firmware version of the device.
    fn get_firmware_version(device: &HidDevice) -> Result<String, Error> {
        let mut buf = [0; 17];
        device.get_feature_report(buf.as_mut())?;
        let version =
            String::from_utf8_lossy(&buf[Self::get_firmware_version_offset()..]).into_owned();
        Ok(version)
    }

    /// Send a raw command to the device.
    fn send_feature_report(
        device: &HidDevice,
        raw: Self::FeatureReportBuffer,
    ) -> Result<(), Error> {
        device.send_feature_report(raw.as_ref())?;
        Ok(())
    }

    /// Send a raw command to the device and receive a response. The response is written to the
    /// same buffer as the command.
    fn get_feature_report(
        device: &HidDevice,
        mut raw: Self::FeatureReportBufferMut,
    ) -> Result<(), Error> {
        device.get_feature_report(raw.as_mut())?;
        Ok(())
    }
}

/// Firmware implementation for the first version of the Stream Deck firmware.
pub struct FirmwareV1;

impl Firmware for FirmwareV1 {
    type FeatureReportBuffer = [u8; 17];
    type FeatureReportBufferMut = [u8; 17];

    fn get_firmware_version_offset() -> usize {
        5
    }
}

pub struct FirmwareMini;

impl Firmware for FirmwareMini {
    type FeatureReportBuffer = [u8; 17];
    type FeatureReportBufferMut = [u8; 17];

    fn get_firmware_version_offset() -> usize {
        5
    }
}

/// Firmware implementation for the second version of the Stream Deck firmware.
pub struct FirmwareV2;

impl Firmware for FirmwareV2 {
    type FeatureReportBuffer = [u8; 32];
    type FeatureReportBufferMut = [u8; 32];

    fn get_firmware_version_offset() -> usize {
        6
    }
}
