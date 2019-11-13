# Astra SDK bindings for Rust

Rust bindings to the [Orbbec Astra Sdk](https://orbbec3d.com/develop/)

## Prerequisites to use this library

* Download and install the [Orbbec Astra Sdk](https://orbbec3d.com/develop/)
* make sure you have these envs in your .profile

```bash
# adjust astra home to your astra_sdk path
export ASTRA_HOME=$HOME/astra
export ASTRA_SDK_INCLUDE=$ASTRA_HOME/include
export ASTRA_SDK_LIB=$ASTRA_HOME/lib
# this is so that rust executables know where to find the astra libs
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$ASTRA_SDK_LIB
```

### Example

Starting with an empty project

```bash
cargo init --bin
```

add dependency

```toml
[dependecies]
astra = { git = "https://github.com/rustmain/astra-rust" }
```

Get first 200 body frames

```rust
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
```

