static ELGATO_VENDOR_ID: u16 = 0x0fd9;
// perhaps we also need pid - most likely
static PID_STREAMDECK_MINI: u16 = 0x0063;

static COMMAND_REV1_FIRMWARE_VERSION: u8 = 0x04;
static COMMAND_REV1_BRIGHTNESS: [u8; 5] = [0x05, 0x55, 0xaa, 0xd1, 0x01];

fn main() {
    
    let hidapi = hidapi::HidApi::new().unwrap();

    hidapi.device_list().for_each(|dev| {
        if dev.vendor_id() == ELGATO_VENDOR_ID {
            println!("{:?}", dev.path());
        }   
    });

    // let devices = hidapi.device_list().filter(|device_info| {
    //     device_info.vendor_id() == ELGATO_VENDOR_ID
    //         && device_info.product_id() == PID_STREAMDECK_MINI
    // }).collect::<Vec<_>>();
    // println!("{:?}", devices);

    
    // let device = devices.first().unwrap();
    // let device = device.open_device(&hidapi).unwrap();

    // // attempt to set display brightness
    // let mut cmd: [u8; 17] = [0; 17];
    // cmd[0..5].copy_from_slice(&COMMAND_REV1_BRIGHTNESS);
    // cmd[5] = 100;
    // device.send_feature_report(&cmd).unwrap();
}