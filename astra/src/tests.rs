use super::*;

#[test]
fn test_body() -> Result<()> {
    let system = get_system();
    let mut sensor = system.sensor.lock().unwrap();
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
    let system = get_system();
    let mut sensor = system.sensor.lock().unwrap();
    let mut index = 0;
    while index < 20 {
        if sensor.update().is_ok() {
            if let Ok(bytes) = sensor.get_color_bytes() {
                index += 1;
            }
        }
    }
    Ok(())
}

#[test]
fn test_masked_color() -> Result<()> {
    let system = get_system();
    let mut sensor = system.sensor.lock().unwrap();
    sensor.start_masked_color_stream()?;
    let mut index = 0;
    while index < 20 {
        if let Ok(frame) = sensor.update() {
            if let Ok((width, height, byte_length, bytes)) = sensor.get_masked_color_bytes() {
                index += 1;
            }
        }
    }
    Ok(())
}

#[test]
fn test_depth() -> Result<()> {
    let system = get_system();
    {
        let mut sensor = system.sensor.lock().unwrap();
        sensor.start_depth_stream()?;
    }
    let mut index = 0;
    while index < 20 {
        let mut sensor = system.sensor.lock().unwrap();
        if let Ok(frame) = sensor.update() {
            if let Ok((width, height, byte_length, bytes)) = sensor.get_depth_bytes() {
                index += 1;

                println!(
                    "width: {}, height: {}, w*h: {}, byte_length: {}",
                    width,
                    height,
                    width * height,
                    byte_length
                );
            }
        }
    }
    Ok(())
}
