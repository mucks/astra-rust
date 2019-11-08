use super::types::StreamSet;
use crate::util::astra_status_to_result;
use model::Result;
use std::ffi::CString;

pub fn get_stream_set() -> Result<StreamSet> {
    unsafe {
        let mut stream_set =
            Box::into_raw(Box::new(sys::_astra_streamsetconnection { _unused: [] })) as StreamSet;
        let path = CString::new("device/default").unwrap();
        let status = sys::astra_streamset_open(path.as_ptr(), &mut stream_set);
        astra_status_to_result(status.into(), stream_set)
    }
}

pub fn close_stream_set(stream_set: &mut StreamSet) -> Result<()> {
    unsafe {
        let status = sys::astra_streamset_close(stream_set);
        astra_status_to_result(status.into(), ())
    }
}
