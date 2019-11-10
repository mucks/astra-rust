#[derive(Clone, Hash, PartialEq, Eq, Debug)]
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
        unsafe { std::mem::transmute(joint_type) }
    }
}
