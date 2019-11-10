use super::types::*;
use crate::util::astra_status_to_result;
use model::ImageFrameType::{self, *};
use model::{Error, Result};

pub fn get_img_frame(img_frame_type: ImageFrameType, frame: AstraFrame) -> Result<ImageFrame> {
    unsafe {
        let mut img_frame: ImageFrame = std::ptr::null_mut();

        let status = match img_frame_type {
            Color => sys::astra_frame_get_colorframe(frame, &mut img_frame),
            MaskedColor => sys::astra_frame_get_maskedcolorframe(frame, &mut img_frame),
            Depth => sys::astra_frame_get_depthframe(frame, &mut img_frame),
            _ => return Err(Error::GetImgFrameError),
        };
        astra_status_to_result(status.into(), img_frame)
    }
}

pub fn get_img_frame_index(img_frame: ImageFrame) -> Result<i32> {
    let mut frame_index = 0_i32;
    unsafe {
        let status = sys::astra_imageframe_get_frameindex(img_frame, &mut frame_index);
        astra_status_to_result(status.into(), frame_index)
    }
}

pub fn get_img_frame_byte_length(img_frame: ImageFrame) -> Result<usize> {
    let mut byte_length = 0;
    unsafe {
        let status = sys::astra_imageframe_get_data_byte_length(img_frame, &mut byte_length);
        astra_status_to_result(status.into(), byte_length as usize)
    }
}

//tmp
pub fn get_masked_color_frame_byte_length(img_frame: ImageFrame) -> Result<usize> {
    let mut byte_length = 0;
    unsafe {
        let status = sys::astra_maskedcolorframe_get_data_byte_length(img_frame, &mut byte_length);
        astra_status_to_result(status.into(), byte_length as usize)
    }
}

pub fn get_img_frame_dimensions(img_frame: ImageFrame) -> Result<(u32, u32)> {
    let mut metadata = sys::astra_image_metadata_t::default();
    unsafe {
        let status = sys::astra_imageframe_get_metadata(img_frame, &mut metadata);
        astra_status_to_result(status.into(), (metadata.width, metadata.height))
    }
}
