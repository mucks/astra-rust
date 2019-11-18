#[derive(Clone, Hash, PartialEq, Eq, Debug, Copy)]
pub enum JointType {
    Head,
    ShoulderSpine,
    LeftShoulder,
    LeftElbow,
    LeftHand,
    RightShoulder,
    RightElbow,
    RightHand,
    MidSpine,
    BaseSpine,
    LeftHip,
    LeftKnee,
    LeftFoot,
    RightHip,
    RightKnee,
    RightFoot,
    LeftWrist,
    RightWrist,
    Neck,
    Unknown = 255,
}

#[cfg(feature = "godot")]
impl gdnative::ToVariant for JointType {
    fn to_variant(&self) -> gdnative::Variant {
        gdnative::Variant::from_u64(*self as u64)
    }
}

impl Default for JointType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for JointType {
    fn from(joint_type: u8) -> Self {
        unsafe { std::mem::transmute(joint_type) }
    }
}
