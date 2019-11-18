#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BodyStatus {
    NotTracking,
    Lost,
    TrackingStarted,
    Tracking,
}

impl Default for BodyStatus {
    fn default() -> Self {
        Self::NotTracking
    }
}

#[cfg(feature = "godot")]
impl gdnative::ToVariant for BodyStatus {
    fn to_variant(&self) -> gdnative::Variant {
        gdnative::Variant::from_u64(*self as u64)
    }
}

impl From<u8> for BodyStatus {
    fn from(body_status: u8) -> Self {
        unsafe { std::mem::transmute(body_status) }
    }
}
