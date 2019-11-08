#[derive(FromPrimitive, Clone, Hash, PartialEq, Eq, Debug)]
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

impl From<u8> for JointType {
    fn from(joint_type: u8) -> Self {
        num::FromPrimitive::from_u8(joint_type)
            .unwrap_or_else(|| panic!("could not parse joint type: {}", joint_type))
    }
}
