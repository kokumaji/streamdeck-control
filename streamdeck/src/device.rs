//! Device-related functionality.

use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use hidapi::HidDevice;

use crate::firmware::{Firmware, FirmwareV1};
use crate::hid::{get_device, get_device_from_path};
use crate::Error;

/// Vendor IDs for Stream Deck devices.
pub mod vendors {
    pub const ELGATO: u16 = 0x0fd9;
}

/// Device IDs for Stream Deck devices.
pub mod device_ids {
    pub const ORIGINAL: u16 = 0x0060;
    pub const ORIGINAL_V2: u16 = 0x006d;
    pub const MINI: u16 = 0x0063;
    pub const XL: u16 = 0x006c;
    pub const MK2: u16 = 0x0080;
    pub const REVISED_MINI: u16 = 0x0090;
}

/// A Stream Deck device.
#[derive(Debug, Clone)]
pub struct Device<I: DeviceInfo<F>, F: Firmware> {
    inner: Arc<HidDevice>,
    buf: Arc<Mutex<[u8; 32]>>,
    __device: PhantomData<F>,
    __info: PhantomData<I>,
}

impl<I, F> Device<I, F>
where
    I: DeviceInfo<F>,
    F: Firmware,
{
    /// Open a Stream Deck device.
    pub fn open() -> Result<Self, Error> {
        Ok(Self {
            inner: get_device(I::vendor_id(), I::product_id())?.into(),
            buf: Arc::new(Mutex::new([0; 32])),
            __device: PhantomData,
            __info: PhantomData,
        })
    }

    /// Open a Stream Deck device by path.
    pub fn open_path(path: String) -> Result<Self, Error> {
        Ok(Self {
            inner: get_device_from_path(I::vendor_id(), I::product_id(), path)?.into(),
            buf: Arc::new(Mutex::new([0; 32])),
            __device: PhantomData,
            __info: PhantomData,
        })
    }

    /// Write data to the firmware buffer.
    fn write_command(&self, command: &[u8], args: &[u8]) -> Result<(), Error> {
        let mut buf = *self.buf.lock().map_err(|_| Error::MutexLockError)?;

        // check bounds
        if command.len() + args.len() > F::buffer_size() {
            return Err(Error::CommandSizeError);
        }

        // copy into buffer
        buf.copy_from_slice(command);
        buf[command.len()..command.len() + args.len()].copy_from_slice(args);

        // fill the rest with zeros
        for i in command.len() + args.len()..F::buffer_size() {
            buf[i] = 0;
        }

        Ok(())
    }

    /// Send a command to the device.
    fn send_command(&self, command: &[u8], data: &[u8]) -> Result<(), Error> {
        self.write_command(command, data)?;
        let buf = *self.buf.lock().map_err(|_| Error::MutexLockError)?;
        Ok(self.inner.send_feature_report(&buf)?)
    }

    /// Send a command to the device and read the response.
    fn read_command(&self, command: &[u8], data: &[u8]) -> Result<(), Error> {
        self.write_command(command, data)?;
        let mut buf = *self.buf.lock().map_err(|_| Error::MutexLockError)?;
        Ok(self.inner.get_feature_report(&mut buf).map(|_| ())?)
    }

    /// Sets the brightness of the device.
    pub fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        self.send_command(&F::brightness_command(), &[brightness])
    }
}

/// A trait defining the device info.
pub trait DeviceInfo<F: Firmware> {
    /// Get the vendor ID of the device.
    fn vendor_id() -> u16;

    /// Get the product ID of the device.
    fn product_id() -> u16;
}

/// A utility trait for connecting to devices. `Self::open()` is identical to `Device::<Self, F>::open()`.
pub trait Connect<F>: DeviceInfo<F>
where
    Self: Sized,
    F: Firmware,
{
    /// Open a device.
    fn open() -> Result<Device<Self, F>, Error> {
        Device::<Self, F>::open()
    }

    /// Open a device by path.
    fn open_path(path: String) -> Result<Device<Self, F>, Error> {
        Device::<Self, F>::open_path(path)
    }
}

/// Blanket implementation for all devices.
impl<I, F> Connect<F> for I
where
    I: DeviceInfo<F>,
    F: Firmware,
{
}

/// A Stream Deck Mini device.
pub struct StreamDeckMini;

impl DeviceInfo<FirmwareV1> for StreamDeckMini {
    fn vendor_id() -> u16 {
        vendors::ELGATO
    }

    fn product_id() -> u16 {
        device_ids::MINI
    }
}
