#[derive(FromPrimitive, Debug, Clone, PartialEq)]
pub enum BodyFeatures {
    TrackingSegmentation = 0,
    TrackingJoints = 1,
    TrackingHandPoses = 3,
}

impl From<u32> for BodyFeatures {
    fn from(features: u32) -> Self {
        num::FromPrimitive::from_u32(features)
            .unwrap_or_else(|| panic!("could not parse body features: {}", features))
    }
}
