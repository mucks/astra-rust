use super::*;

#[test]
fn test_body_stream() {
    init();
    let sensor = get_sensor();
    let reader = get_reader(sensor);
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
    terminate();
}
