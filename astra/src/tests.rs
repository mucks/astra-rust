use super::*;

#[test]
#[ignore]
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

#[test]
#[ignore]
fn test_infrared() -> Result<()> {
    let mut sensor = Sensor::new();
    sensor.init()?;
    sensor.start_infrared_stream()?;
    let mut index = 0;
    while index < 20 {
        if let Ok(frame) = sensor.update() {
            if let Ok(ir) = sensor.get_infrared_bytes(&frame) {
                println!("{:?}", ir);
            }
        }
        index = sensor.get_infrared_index();
    }
    Ok(())
}

#[test]
#[ignore]
fn masked_color() -> Result<()> {
    let mut sensor = Sensor::new();
    sensor.init()?;
    sensor.start_masked_color_stream()?;
    let mut index = 0;
    while index < 20 {
        if let Ok(frame) = sensor.update() {
            let time = std::time::SystemTime::now();
            if let Ok(bytes) = sensor.get_masked_color_bytes(&frame) {}
            println!("{:?}", time.elapsed().unwrap().as_micros());
        }
        index = sensor.get_masked_color_index();
    }
    Ok(())
}

#[test]
fn body() -> Result<()> {
    let mut sensor = Sensor::new();
    sensor.init()?;
    sensor.start_body_stream()?;
    let mut index = 0;
    while index < 200 {
        if let Ok(frame) = sensor.update() {
            let time = std::time::SystemTime::now();
            if let Ok(bodies) = sensor.get_bodies(&frame) {
                println!("{:?}", time.elapsed().unwrap().as_micros());
            }
        }
        index = sensor.get_body_index();
    }
    Ok(())
}
