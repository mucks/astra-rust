use super::types::{AstraFrame, ColorFrame};
use crate::util::astra_status_to_result;
use model::{Error, Result};

//ByteArray::new().write().as_mut_ptr()
pub fn get_color_byte_array(color_frame: ColorFrame, ptr: *mut u8) -> Result<()> {
    unsafe {
        let status = sys::astra_colorframe_copy_data(color_frame, ptr);
        astra_status_to_result(status.into(), ())
    }
}

pub fn get_color_bytes(color_frame: ColorFrame, byte_length: u32) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    data.resize(byte_length as usize, 0);
    unsafe {
        sys::astra_colorframe_copy_data(color_frame, data.as_mut_ptr());
    }
    data
}
