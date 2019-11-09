use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::time::Duration;
use std::{mem, thread};

use super::sensor::Sensor;

#[derive(Clone)]
pub struct System {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    pub sensor: Arc<Mutex<Sensor>>,
}

pub fn get_system() -> System {
    // Initialize it to a null value
    static mut SINGLETON: *const System = 0 as *const System;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = System {
                sensor: Arc::new(Mutex::new(Sensor::new().unwrap())),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}
