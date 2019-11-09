use super::img::*;
use super::types::{AstraFrame, ColorFrame};
use crate::util::astra_status_to_result;
use model::{Error, Result};

pub fn get_color_bytes(color_frame: ColorFrame) -> Result<(u32, u32, usize, Vec<u8>)> {
    let (width, height) = get_img_frame_dimensions(color_frame)?;
    let byte_length = get_img_frame_byte_length(color_frame)?;

    let mut data: Vec<u8> = Vec::new();
    data.resize(byte_length as usize, 0);
    unsafe {
        let status = sys::astra_colorframe_copy_data(color_frame, data.as_mut_ptr());
        astra_status_to_result(status.into(), (width, height, byte_length, data))
    }
}
