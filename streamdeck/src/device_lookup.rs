use lazy_static::lazy_static;
use std::sync::Mutex;
use hidapi::{HidApi, HidDevice};

/// Custom error type.
#[derive(Debug)]
pub enum Error {
    HidError(hidapi::HidError),
    DeviceNotFound,
    MutexError,
}

impl From<hidapi::HidError> for Error {
    fn from(err: hidapi::HidError) -> Self {
        Error::HidError(err)
    }
}

lazy_static! {
    static ref HID_API: Mutex<HidApi> = Mutex::new(HidApi::new().expect("Failed to create HidApi instance"));
}

pub fn get_hid_device(vendor_id: u16, product_id: u16) -> Result<HidDevice, Error> {
    let mut api = HID_API.lock().map_err(|_| Error::MutexError)?;
    api.refresh_devices().map_err(Error::HidError)?;

    let devices = api.device_list().filter(|device_info| {
        device_info.vendor_id() == vendor_id && device_info.product_id() == product_id
    }).collect::<Vec<_>>();

    let device = devices.first().unwrap().open_device(&api);

    device.map_err(Error::HidError)

}