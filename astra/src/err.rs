use super::FrameType;
use super::StreamType;

#[derive(Debug)]
pub enum Error {
    SensorAlreadyStartedError,
    SensorNotStartedError,
    StreamAlreadyStartedError(StreamType),
    StreamNotStartedError(StreamType),
    CouldNotGetFrameError(FrameType),
    NoNewFrameError,
}
