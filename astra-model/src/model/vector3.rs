#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[cfg(feature = "godot")]
impl gdnative::ToVariant for Vector3 {
    fn to_variant(&self) -> gdnative::Variant {
        gdnative::Vector3::new(self.x, self.y, self.z).to_variant()
    }
}

#[cfg(feature = "godot")]
impl gdnative::FromVariant for Vector3 {
    fn from_variant(variant: &gdnative::Variant) -> Option<Self> {
        use gdnative::FromVariant;
        match gdnative::Vector3::from_variant(variant) {
            Some(v) => Some(Vector3 {
                x: v.x,
                y: v.y,
                z: v.z,
            }),
            None => None,
        }
    }
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
