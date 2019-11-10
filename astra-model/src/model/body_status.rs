#[derive(Debug, Clone, PartialEq)]
pub enum BodyStatus {
    NotTracking,
    Lost,
    TrackingStarted,
    Tracking,
}

impl From<u8> for BodyStatus {
    fn from(body_status: u8) -> Self {
        unsafe { std::mem::transmute(body_status) }
    }
}
