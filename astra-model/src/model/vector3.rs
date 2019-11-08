#[derive(Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<sys::astra_vector3f_t> for Vector3 {
    fn from(v: sys::astra_vector3f_t) -> Self {
        Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
