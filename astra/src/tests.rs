use super::*;

#[test]
fn test_all_streams() -> Result<()> {
    let mut sensor = Sensor::new();
    sensor.init()?;
    sensor.start_body_stream()?;
    sensor.start_color_stream()?;
    sensor.start_depth_stream()?;
    sensor.start_masked_color_stream()?;
    let mut index = 0;
    while index < 20 {
        if let Ok(frame) = sensor.update() {
            if let Ok(color) = sensor.get_color_bytes(&frame) {
                println!("got color")
            }
            if let Ok(bodies) = sensor.get_bodies(&frame) {
                println!("got_body");
            }
            index += 1;
            sensor.stop_all()?;
            sensor.init()?;
            sensor.start_body_stream()?;
            sensor.start_color_stream()?;
        }
    }
    sensor.stop_all()?;
    Ok(())
}
