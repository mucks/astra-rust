use crate::AstraStatus;
use crate::FrameType;
use crate::StreamType;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    StreamAlreadyStartedError(StreamType),
    StreamNotStartedError(StreamType),
    CouldNotGetFrameError(FrameType),
    NoNewFrameError,
    NoUpdateCallError,
    AstraStatusError(AstraStatus),
    GetImgFrameError,
    SensorNotInitializedError,
    SensorAlreadyInitializedError,
}

impl From<&Error> for Error {
    fn from(error: &Error) -> Self {
        error.clone()
    }
}
