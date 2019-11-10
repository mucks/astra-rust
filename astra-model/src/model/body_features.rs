#[derive(Debug, Clone, PartialEq)]
pub enum BodyFeatures {
    TrackingSegmentation = 0,
    TrackingJoints = 1,
    TrackingHandPoses = 3,
}

impl From<u32> for BodyFeatures {
    fn from(features: u32) -> Self {
        unsafe { std::mem::transmute(features as u8) }
    }
}
