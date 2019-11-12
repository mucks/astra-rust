fn main() -> astra::Result<()> {
    // will stop automatically if fallen out of context
    let mut sensor = astra::Sensor::new();
    // will return err if already initialized
    sensor.init()?;
    // will return err if already started
    sensor.start_body_stream()?;
    let mut index = 0;
    while index < 200 {
        // have to call update in order to get the frame which you need to get data
        if let Ok(frame) = sensor.update() {
            if let Ok(bodies) = sensor.get_bodies(&frame) {
                for body in bodies {
                    for (jt, joint) in body.joints {
                        println!("{:?}: {:?}", jt, joint.world_position);
                    }
                }
            }
        }
        index = sensor.get_body_index();
    }
    Ok(())
}
