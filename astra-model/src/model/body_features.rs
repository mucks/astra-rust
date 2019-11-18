#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BodyFeatures {
    TrackingSegmentation = 0,
    TrackingJoints = 1,
    TrackingHandPoses = 3,
}

#[cfg(feature = "godot")]
impl gdnative::ToVariant for BodyFeatures {
    fn to_variant(&self) -> gdnative::Variant {
        gdnative::Variant::from_u64(*self as u64)
    }
}

impl Default for BodyFeatures {
    fn default() -> Self {
        Self::TrackingJoints
    }
}

impl From<u32> for BodyFeatures {
    fn from(features: u32) -> Self {
        unsafe { std::mem::transmute(features as u8) }
    }
}
