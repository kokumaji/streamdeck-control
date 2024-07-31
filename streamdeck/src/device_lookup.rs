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

pub fn get_hid_device(api: &HidApi, vendor_id: u16, product_id: u16) -> Result<HidDevice, Error> {
    let devices = api.device_list().filter(|di| {
        di.vendor_id() == vendor_id && di.product_id() == product_id
    }).collect::<Vec<_>>();

    let device = devices.first().unwrap().open_device(&api);

    device.map_err(Error::HidError)

}

pub fn get_hid_device_from_path(api: &HidApi, vendor_id: u16, product_id: u16, path: String) -> Result<HidDevice, Error> {
    let devices = api.device_list().filter(|di| {
        di.vendor_id() == vendor_id && di.product_id() == product_id && di.path().to_string_lossy() == path
    }).collect::<Vec<_>>();

    let device = devices.first().unwrap().open_device(&api);
    device.map_err(Error::HidError)

}