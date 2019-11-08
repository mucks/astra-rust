use super::types::{Reader, Stream};
use model::StreamType;

pub fn start_stream(reader: Reader, stream_type: StreamType) -> Stream {
    use self::StreamType::*;
    unsafe {
        let mut stream = Box::into_raw(Box::new(sys::_astra_streamconnection::default())) as Stream;
        match stream_type {
            Body => sys::astra_reader_get_bodystream(reader, &mut stream),
            Color => sys::astra_reader_get_colorstream(reader, &mut stream),
            MaskedColor => sys::astra_reader_get_maskedcolorstream(reader, &mut stream),
        };
        sys::astra_stream_start(stream);
        stream
    }
}

pub fn stop_stream(stream: Stream) {
    unsafe {
        sys::astra_stream_stop(stream);
    }
}
