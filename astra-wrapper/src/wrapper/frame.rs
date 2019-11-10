use super::types::{AstraFrame, Reader};

pub fn close_frame(frame: &mut AstraFrame) {
    unsafe {
        sys::astra_reader_close_frame(frame);
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
