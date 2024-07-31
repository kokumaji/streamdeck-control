use image::{ImageBuffer, ImageReader, Rgb};
use rand::Rng;

static REPORT_IMG_HEADER_LENGTH: usize = 16;

const TOTAL_IMAGE_BYTES: usize = IMAGE_WIDTH * IMAGE_HEIGHT * 3;
const IMAGE_WIDTH: usize = 80;
const IMAGE_HEIGHT: usize = 80;

const PACKET_SIZE: usize = 1024;

const MINI_IMAGE_BASE: [u8; 54] = [
    0x42, 0x4d, 0xf6, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00,
    0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xc0, 0x3c, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

static ELGATO_VENDOR_ID: u16 = 0x0fd9;
// perhaps we also need pid - most likely
static PID_STREAMDECK_MINI: u16 = 0x0063;

// original v1 firmware for now 
// v2 requires byte 5-6 to contain chunk size, sequence is byte 7-8
fn create_image_header(buf: &mut [u8], key: u8, sequence: u16, is_last: bool, payload: usize) {
    
    buf[0] = 0x02;
    buf[1] = 0x01;
    buf[2..4].copy_from_slice(&sequence.to_le_bytes());
    buf[4] = if is_last { 1 } else { 0 };
    buf[5] = key;
    
}

fn image_to_u8_array(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    img.iter().copied().collect()
}

fn main() {

    let hidapi = hidapi::HidApi::new().unwrap();
    let devices = hidapi.device_list().filter(|device_info| {
        device_info.vendor_id() == ELGATO_VENDOR_ID
            && device_info.product_id() == PID_STREAMDECK_MINI
    }).collect::<Vec<_>>();
    
    let device = devices.first().unwrap();
    let device = device.open_device(&hidapi).unwrap();

    // Define the image dimensions and square properties
    let width = IMAGE_WIDTH;
    let height = IMAGE_HEIGHT;
    let square_size = 50;

    let filePath = "./image/example.bmp";

    // Create a new image buffer with white background
    let mut imgRead = ImageReader::open(filePath).unwrap().decode().expect("Failed to read Image.");
    let mut img = imgRead.into_rgb8();

    // Draw the square at the center of the image
    // let start_x = (width - square_size) / 2;
    // let start_y = (height - square_size) / 2;

    // for x in start_x..(start_x + square_size) {
    //     for y in start_y..(start_y + square_size) {
    //         let r = rand::thread_rng().gen_range(0..255);
    //         let g = rand::thread_rng().gen_range(0..255);
    //         let b = rand::thread_rng().gen_range(0..255);
    //         img.put_pixel(x as u32, y as u32, Rgb([b, g, r]));
    //     }
    // }

    let image = image_to_u8_array(&img);

    let mut bgr_buffer: Vec<u8> = Vec::with_capacity((width * height * 3) as usize);

    for x in (0..width).rev() {
        for y in 0..height {
            let pixel = img.get_pixel(x as u32, y as u32);
            bgr_buffer.push(pixel[2]); // Blue
            bgr_buffer.push(pixel[1]); // Green
            bgr_buffer.push(pixel[0]); // Red
        }
    }

    for sd_key in 1..7 {
        let mut buf = vec![0u8; PACKET_SIZE];

        let mut sequence = 0;
        let mut offset = 0;
        let maxdatalen = buf.len() - REPORT_IMG_HEADER_LENGTH;
    
        while offset < img.len() {
            let mut take = (bgr_buffer.len() - offset).min(maxdatalen);
            let mut start = REPORT_IMG_HEADER_LENGTH;
    
            if sequence == 0 {
                buf[start..start + MINI_IMAGE_BASE.len()].copy_from_slice(&MINI_IMAGE_BASE);
                take = (img.len() - offset).min(maxdatalen - MINI_IMAGE_BASE.len());
                start += MINI_IMAGE_BASE.len();
            }
    
            let is_last = take == img.len() - offset;
            create_image_header(&mut buf, sd_key, sequence, is_last, take);
    
            buf[start..start + take].copy_from_slice(&bgr_buffer[offset..offset+take]);
    
            device.write(&buf);
    
            sequence += 1;
            offset += take;
    
        }

    }

    let mut buf = [0u8; 64];
    loop {
        match device.read_timeout(&mut buf, 5000) {
            Ok(size) if size > 0 => {
                let mut key_pressed = 0;

                for (index, &value) in buf.iter().skip(1).enumerate() {
                    if value == 1 {
                        key_pressed = index + 1;
                        break;
                    }
                }
            

                if (key_pressed > 0) {
                    println!("Received Key Press {:?}", key_pressed);
                } else {
                    println!("Received Key Release Event");
                }

            }
            Ok(_) => {
                println!("No data received within timeout period.");
            }
            Err(_) => todo!(),
        }
    }

    // create_image_header(&mut buf, sd_key, 1, false);

    // let image_start = REPORT_IMG_HEADER_LENGTH + MINI_IMAGE_BASE.len();
    // buf[REPORT_IMG_HEADER_LENGTH..image_start].copy_from_slice(&MINI_IMAGE_BASE);
    // buf[image_start..image_start + 7749].copy_from_slice(&img[0..7749]);

    // device.write(&buf);

    // create_image_header(&mut buf, sd_key, 2, true);
    // buf[REPORT_IMG_HEADER_LENGTH..REPORT_IMG_HEADER_LENGTH + 7803].copy_from_slice(&img[7749..15552]);
    // device.write(&buf);
}
