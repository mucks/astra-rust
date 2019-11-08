use super::*;

#[test]
fn test_body_stream() {
    let mut sensor = sensor::Sensor::new();
    if sensor.start().is_ok() && sensor.start_color_stream().is_ok() {
        let mut index = -1;
        while index < 100 {
            update();
            if let Ok(_bodies) = sensor.get_bodies() {
                index += 1;
            }
        }
    }
}

#[test]
fn test_color_stream() {
    let mut sensor = sensor::Sensor::new();
    if sensor.start().is_ok() && sensor.start_color_stream().is_ok() {
        let mut index = -1;
        while index < 100 {
            update();
            if let Ok(_bytes) = sensor.get_color_bytes() {
                index += 1;
            }
        }
    }
}
