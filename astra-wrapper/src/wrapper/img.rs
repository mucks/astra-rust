use super::types::*;
use crate::util::astra_status_to_result;
use crate::*;
use model::StreamType::{self, *};
use model::{AstraStatus, Error, Result};

pub fn get_bytes(img_frame: ImageFrame) -> Result<(u32, u32, usize, Vec<u8>)> {
    let (width, height) = get_img_frame_dimensions(img_frame)?;
    let byte_length = get_img_frame_byte_length(img_frame)?;

    let mut data: Vec<u8> = Vec::new();
    data.resize(byte_length as usize, 0);
    unsafe {
        let status = sys::astra_imageframe_copy_data(img_frame, data.as_mut_ptr() as *mut _);
        astra_status_to_result(status.into(), (width, height, byte_length, data))
    }
}
pub fn copy_bytes(img_frame: ImageFrame, ptr: *mut u8) -> Result<()> {
    unsafe {
        let status = sys::astra_imageframe_copy_data(img_frame, ptr as *mut _);
        astra_status_to_result(status.into(), ())
    }
}

pub fn get_img_frame(stream_type: StreamType, frame: AstraFrame) -> Result<ImageFrame> {
    unsafe {
        let mut img_frame: ImageFrame = std::ptr::null_mut();

        let status = match stream_type {
            Color => sys::astra_frame_get_colorframe(frame, &mut img_frame),
            MaskedColor => sys::astra_frame_get_maskedcolorframe(frame, &mut img_frame),
            Depth => sys::astra_frame_get_depthframe(frame, &mut img_frame),
            Infrared => sys::astra_frame_get_infraredframe(frame, &mut img_frame),
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

pub fn get_img_frame_dimensions(img_frame: ImageFrame) -> Result<(u32, u32)> {
    let mut metadata = sys::astra_image_metadata_t::default();
    unsafe {
        let status = sys::astra_imageframe_get_metadata(img_frame, &mut metadata);
        astra_status_to_result(status.into(), (metadata.width, metadata.height))
    }
}
