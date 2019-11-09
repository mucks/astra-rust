use crate::AstraStatus;
use crate::FrameType;
use crate::StreamType;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SensorAlreadyStartedError,
    SensorNotStartedError,
    StreamAlreadyStartedError(StreamType),
    StreamNotStartedError(StreamType),
    CouldNotGetFrameError(FrameType),
    NoNewFrameError,
    NoUpdateCallError,
    AstraStatusError(AstraStatus),
    GetImgFrameError,
}
