#[derive(FromPrimitive, Debug, Clone, PartialEq)]
pub enum JointStatus {
    NotTracked,
    LowConfidence,
    Tracked,
}

impl From<u8> for JointStatus {
    fn from(joint_status: u8) -> Self {
        num::FromPrimitive::from_u8(joint_status)
            .unwrap_or_else(|| panic!("could not parse joint status: {}", joint_status))
    }
}
