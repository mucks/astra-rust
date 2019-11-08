use super::*;

#[test]
#[ignore]
fn test_body() -> Result<(), Error> {
    let mut sensor = Sensor::new();
    sensor.start()?;
    sensor.start_body_stream()?;

    let mut index = 0;
    while index < 20 {
        sensor.update();
        if let Ok(bodies) = sensor.get_bodies() {
            index += 1;
        }
    }
    Ok(())
}

#[test]
fn test_color() -> Result<(), Error> {
    let mut sensor = Sensor::new();
    sensor.start()?;
    sensor.start_color_stream()?;
    let mut index = 0;
    while index < 20 {
        sensor.update();
        if let Ok(bytes) = sensor.get_color_bytes() {
            index += 1;
        }
    }
    Ok(())
}
