#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl From<sys::astra_vector2f_t> for Vector2 {
    fn from(v: sys::astra_vector2f_t) -> Self {
        Vector2 { x: v.x, y: v.y }
    }
}
