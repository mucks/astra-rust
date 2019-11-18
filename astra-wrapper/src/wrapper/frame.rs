use super::types::{AstraFrame, Reader};
use crate::util::astra_status_to_result;
use model::Result;

pub fn close_frame(frame: &mut AstraFrame) -> Result<()> {
    unsafe {
        let status = sys::astra_reader_close_frame(frame);
        astra_status_to_result(status.into(), ())
    }
}

pub fn get_frame(reader: Reader) -> Option<AstraFrame> {
    unsafe {
        let mut frame: AstraFrame = std::ptr::null_mut();
        let rc = sys::astra_reader_open_frame(reader, 0, &mut frame);

        if rc == sys::astra_status_t_ASTRA_STATUS_SUCCESS {
            Some(frame)
        } else {
            None
        }
    }
}
