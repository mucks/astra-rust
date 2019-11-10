use super::types::{Reader, Stream};
use crate::util::astra_status_to_result;
use model::{Result, StreamType};

pub fn start_stream(reader: Reader, stream_type: StreamType) -> Result<Stream> {
    use self::StreamType::*;
    unsafe {
        let mut stream: Stream = std::ptr::null_mut();
        match stream_type {
            Body => sys::astra_reader_get_bodystream(reader, &mut stream),
            Color => sys::astra_reader_get_colorstream(reader, &mut stream),
            MaskedColor => sys::astra_reader_get_maskedcolorstream(reader, &mut stream),
            Depth => sys::astra_reader_get_depthstream(reader, &mut stream),
        };
        let status = sys::astra_stream_start(stream);
        astra_status_to_result(status.into(), stream)
    }
}

pub fn stop_stream(stream: Stream) -> Result<()> {
    unsafe {
        let status = sys::astra_stream_stop(stream).into();
        astra_status_to_result(status, ())
    }
}
