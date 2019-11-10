#[derive(Debug, Clone, PartialEq)]
pub enum JointStatus {
    NotTracked,
    LowConfidence,
    Tracked,
}

impl From<u8> for JointStatus {
    fn from(joint_status: u8) -> Self {
        unsafe { std::mem::transmute(joint_status) }
    }
}
