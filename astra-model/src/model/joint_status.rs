#[derive(Debug, Clone, PartialEq, Copy)]
pub enum JointStatus {
    NotTracked,
    LowConfidence,
    Tracked,
}

impl Default for JointStatus {
    fn default() -> Self {
        Self::NotTracked
    }
}

#[cfg(feature = "godot")]
impl gdnative::ToVariant for JointStatus {
    fn to_variant(&self) -> gdnative::Variant {
        gdnative::Variant::from_u64(*self as u64)
    }
}

impl From<u8> for JointStatus {
    fn from(joint_status: u8) -> Self {
        unsafe { std::mem::transmute(joint_status) }
    }
}
