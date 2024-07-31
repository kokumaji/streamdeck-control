//! Defines firmware traits and implementations for Stream Deck devices.

pub const FW_BMP_HEADER: [u8; 54] = [
    0x42, 0x4d, 0xf6, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00,
    0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xc0, 0x3c, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// A trait for firmware information.
pub trait Firmware: Send + Sync {
    /// The offset of the firmware version in the feature report.
    fn version_offset() -> usize;

    /// The size of the buffer.
    fn buffer_size() -> usize;

    /// The brightness command.
    fn brightness_command() -> &'static [u8];
}

/// Firmware implementation for the first version of the Stream Deck firmware.
pub struct FirmwareV1;

impl Firmware for FirmwareV1 {
    fn version_offset() -> usize {
        5
    }

    fn buffer_size() -> usize {
        17
    }

    fn brightness_command() -> &'static [u8] {
        &[0x05, 0x55, 0xaa, 0xd1, 0x01]
    }
}

/// Firmware implementation for the second version of the Stream Deck firmware.
pub struct FirmwareV2;

impl Firmware for FirmwareV2 {
    fn version_offset() -> usize {
        6
    }

    fn buffer_size() -> usize {
        32
    }

    fn brightness_command() -> &'static [u8] {
        &[0x05, 0x55, 0xaa, 0xd1, 0x01]
    }
}
