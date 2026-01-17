use hidapi::HidApi;

const VENDOR_ID: u16 = 0x054c;
const PRODUCT_ID: u16 = 0x09cc;

const MAX_STEERING_ANGLE: f32 = 70.0;

const STEERING_FIZZ: i32 = 0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = HidApi::new()?;
    let controller = api.open(VENDOR_ID, PRODUCT_ID)?;
    let mut input_device = uinput::default()?
        .name("Virtual Gamepad")?
        .version(2 as u16)
        .event(uinput::event::absolute::Position::X)?
        .max(255)
        .min(0)
        .fuzz(STEERING_FIZZ)
        .create()?;
    let mut buffer: [u8; 64] = [0; 64];

    loop {
        let n = controller.read(&mut buffer)?;

        println!("Read {} bytes of data", n);

        let accelo_x = i16::from_le_bytes([buffer[19], buffer[20]]) as f32;
        let accelo_y = i16::from_le_bytes([buffer[21], buffer[22]]) as f32;

        println!("Accel X: {}, Accel Y: {}", accelo_x, accelo_y);

        let theta = accelo_y.atan2(accelo_x).to_degrees() - 90_f32;

        let theta_normalised = (theta / MAX_STEERING_ANGLE).clamp(-1_f32, 1_f32);

        let steering_input = ((127.5 * theta_normalised) + 127.5) as i32;
        println!("Steering Input: {}", steering_input);
        println!("Theta: {}", theta);
        let _ = input_device.position(&uinput::event::absolute::Position::X, steering_input);
        let _ = input_device.synchronize();
    }
}
