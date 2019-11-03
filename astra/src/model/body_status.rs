#[derive(FromPrimitive, Debug, Clone, PartialEq)]
pub enum BodyStatus {
    NotTracking,
    Lost,
    TrackingStarted,
    Tracking,
}

impl From<u8> for BodyStatus {
    fn from(body_status: u8) -> Self {
        num::FromPrimitive::from_u8(body_status)
            .expect(&format!("could not parse body status: {}", body_status))
    }
}
