use super::types::{AstraFrame, ColorFrame};
use crate::util::astra_status_to_result;
use model::{Error, Result};

pub fn get_color_frame(frame: AstraFrame) -> Result<ColorFrame> {
    unsafe {
        let mut color_frame =
            Box::into_raw(Box::new(sys::_astra_imageframe::default())) as ColorFrame;
        let status = sys::astra_frame_get_colorframe(frame, &mut color_frame);
        astra_status_to_result(status.into(), color_frame)
    }
}

pub fn get_color_frame_index(color_frame: ColorFrame) -> i32 {
    unsafe {
        let mut frame_index = 0_i32;
        sys::astra_colorframe_get_frameindex(color_frame, &mut frame_index);
        frame_index
    }
}

pub fn get_color_frame_byte_length(color_frame: ColorFrame) -> usize {
    unsafe {
        let mut byte_length = 0;
        sys::astra_colorframe_get_data_byte_length(color_frame, &mut byte_length);
        byte_length as usize
    }
}
pub fn get_color_frame_dimensions(color_frame: ColorFrame) -> (u32, u32) {
    unsafe {
        let mut metadata = sys::astra_image_metadata_t::default();
        sys::astra_colorframe_get_metadata(color_frame, &mut metadata);
        (metadata.width, metadata.height)
    }
}

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
