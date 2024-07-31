use streamdeck::{Connect, StreamDeckMini};

/// Attempts to open a Stream Deck Mini device and set its brightness to 40.
#[test]
fn set_brightness() {
    let device = StreamDeckMini::open().unwrap();
    device.set_brightness(40).unwrap();
}
