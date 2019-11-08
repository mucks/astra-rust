use super::types::StreamSet;
use std::ffi::CString;

pub fn get_stream_set() -> StreamSet {
    unsafe {
        let mut sensor =
            Box::into_raw(Box::new(sys::_astra_streamsetconnection { _unused: [] })) as StreamSet;
        let path = CString::new("device/default").unwrap();
        sys::astra_streamset_open(path.as_ptr(), &mut sensor);
        sensor
    }
}

pub fn close_stream_set(stream_set: &mut StreamSet) {
    unsafe {
        sys::astra_streamset_close(stream_set);
    }
}
