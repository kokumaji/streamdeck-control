//! HID utilities.

use hidapi::{HidApi, HidDevice};

use crate::Error;

//// Fetches a device by vendor and product ID.
pub fn get_device(vendor_id: u16, product_id: u16) -> Result<HidDevice, Error> {
    HidApi::new()?
        .device_list()
        .filter(|di| di.vendor_id() == vendor_id && di.product_id() == product_id)
        .next()
        .ok_or(Error::DeviceNotFound)
        .and_then(|di| Ok(di.open_device(&HidApi::new()?)?))
}

/// Fetches a device by vendor and product ID and path.
pub fn get_device_from_path(
    vendor_id: u16,
    product_id: u16,
    path: String,
) -> Result<HidDevice, Error> {
    HidApi::new()?
        .device_list()
        .filter(|di| {
            di.vendor_id() == vendor_id
                && di.product_id() == product_id
                && di.path().to_string_lossy() == path
        })
        .next()
        .ok_or(Error::DeviceNotFound)
        .and_then(|di| Ok(di.open_device(&HidApi::new()?)?))
}
