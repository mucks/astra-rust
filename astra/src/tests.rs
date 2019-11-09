use super::*;

#[test]
fn test_body() -> Result<()> {
    let mut sensor = Sensor::new()?;
    sensor.start_body_stream()?;

    let mut index = 0;
    while index < 20 {
        sensor.update()?;
        if let Ok(bodies) = sensor.get_bodies() {
            index += 1;
        }
    }
    Ok(())
}

#[test]
fn test_color() -> Result<()> {
    let mut sensor = Sensor::new()?;
    sensor.start_color_stream()?;
    let mut index = 0;
    while index < 20 {
        if let Ok(frame) = sensor.update_color() {
            if let Ok(bytes) = sensor.get_color_bytes(frame) {
                index += 1;
            }
        }
    }
    Ok(())
}
