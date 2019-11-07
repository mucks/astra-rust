use super::*;
/*
#[test]
#[ignore]
fn test_body_stream() {
    init();
    let mut sensor = get_sensor();
    let mut reader = get_reader(sensor);
    let s = start_body_stream(reader);
    let mut index = -1;
    while index < 100 {
        update();
        if let Some(mut frame) = get_frame(reader) {
            let body_frame = get_body_frame(frame);
            let body_frame_index = get_body_frame_index(body_frame);

            if index != body_frame_index {
                index = body_frame_index;
                get_bodies(body_frame).iter().for_each(|b| {
                    if b.status == crate::model::BodyStatus::Tracking {
                        println!("{:?}", b.center_of_mass)
                    }
                });
            }
            close_frame(&mut frame);
        }
    }
    stop_stream(s);
    close_reader(&mut reader);
    close_sensor(&mut sensor);
    terminate();
}
*/

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
