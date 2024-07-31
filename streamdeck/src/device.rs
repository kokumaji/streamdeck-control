//! Device-related functionality.

use core::panic;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::time::{sleep, Duration};
use hidapi::{DeviceInfo, HidApi, HidDevice, HidError};
use Vendors::ELGATO;

use crate::device_lookup::{get_hid_device, get_hid_device_from_path};
use crate::{error::Error, firmware::{Firmware, FirmwareV1}};

pub enum CommandArg {
    Single(u8),
    Vec(Vec<u8>)
}

pub mod Vendors {
    pub const ELGATO: u16 = 0x0fd9;
}

pub mod StreamDeckId {
    pub const ORIGINAL: u16 = 0x0060;
    pub const ORIGINAL_V2: u16 = 0x006d;
    pub const MINI: u16 = 0x0063;
    pub const XL: u16 = 0x006c;
    pub const MK2: u16 = 0x0080;
    pub const REVISED_MINI: u16 = 0x0090;
}

/// A trait representing a device implementing a particular firmware.
pub trait Device<F>
where
    F: Firmware,
{
    /// Returns the inner HID device.
    fn get_inner(&self) -> MutexGuard<HidDevice>;

    /// Get the firmware version of the device.
    fn get_firmware_version(&self) -> Result<String, Error> {
        F::get_firmware_version(&self.get_inner())
    }

    fn connect(hid_path: Option<String>) -> Self;

    fn send_cmd(&self, command: Vec<u8>, args: Option<CommandArg>) {
        if command.len() > 17 {
            panic!("Command Array too Large.");
        }

        let mut cmd: [u8; 17] = [0; 17];
        cmd[0..command.len()].copy_from_slice(&command);

        if let Some(parsed_args) = args {
            let start = command.len();

            match parsed_args {
                CommandArg::Single(value) => {
                    cmd[start] = value;
                },
                CommandArg::Vec(value) => {
                    if value.len() > cmd.len() - 1 || start + value.len() > cmd.len() {
                        panic!("Command Arguments Array too Large.")
                    }
                    cmd[start..start + value.len()].copy_from_slice(&value); 
                }
            }
        }

        self.get_inner().send_feature_report(&cmd).unwrap();

    }

    /**
     * fades device brightness from start percentage to end percentage in milliseconds
     */
    async fn fade_brightness(&self, start: u8, end: u8, duration: u16, steps: u8) {
        let start = start.clamp(0, 100);
        let end = end.clamp(start, 100);

        let steps = steps.clamp(0, 250);
        let interval = duration as u64 / steps as u64;

        // Determine the step size for brightness change
        let step_size = if start < end {
            (end - start) / steps
        } else {
            (start - end) / steps
        };

        let mut current_brightness = start;

        for _ in 0..steps {
            // Adjust brightness
            self.set_brightness(current_brightness);

            // Wait for the interval before changing to the next step
            sleep(Duration::from_millis(interval)).await;

            // Update brightness for the next step
            if start < end {
                current_brightness = (current_brightness + step_size).clamp(0, 100);
            } else {
                current_brightness = (current_brightness - step_size).clamp(0, 100);
            }
        }

        // Ensure the final brightness is set
        self.set_brightness(end);

    }

    fn set_brightness(&self, mut brightness: u8) {
        // this is v1 only for now
        if brightness > 100 {
            brightness = 100;
        }

        self.send_cmd(COMMAND_REV1_BRIGHTNESS.to_vec(), Some(CommandArg::Single(brightness)))

    }

}

static COMMAND_REV1_BRIGHTNESS: [u8; 5] = [0x05, 0x55, 0xaa, 0xd1, 0x01];
static COMMAND_REV1_RESET: [u8; 2] = [0x0b, 0x63];

/// A Stream Deck Mini device.
pub struct StreamDeckMini {
    hid_device: Arc<Mutex<HidDevice>>
}

impl Device<FirmwareV1> for StreamDeckMini {
    fn get_inner(&self) -> MutexGuard<HidDevice> {
        self.hid_device.lock().unwrap()
    }

    fn connect(hid_path: Option<String>) -> Self  {
        let api = HidApi::new().unwrap();

        let device = match hid_path {
            Some(s) => get_hid_device_from_path(&api, ELGATO, StreamDeckId::MINI, s),
            _ => get_hid_device(&api, ELGATO, StreamDeckId::MINI)
        };

        StreamDeckMini { hid_device: Arc::new(Mutex::new(device.unwrap())) }

    }

}   