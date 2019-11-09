use super::img::*;
use super::types::*;
use crate::util::astra_status_to_result;
use model::AstraStatus;
use model::Result;

pub fn get_depth_data(depth_frame: DepthFrame) -> Result<(u32, u32, usize, Vec<i16>)> {
    let (width, height) = get_img_frame_dimensions(depth_frame)?;
    let depth_length = (width * height) as usize;

    let mut data: Vec<i16> = Vec::new();
    data.resize(depth_length as usize, 0);
    unsafe {
        let status = sys::astra_depthframe_copy_data(depth_frame, data.as_mut_ptr());
        astra_status_to_result(status.into(), (width, height, depth_length, data))
    }
}

pub fn get_depth_bytes(depth_frame: DepthFrame) -> Result<(u32, u32, usize, Vec<u8>)> {
    let (width, height, depth_length, data) = get_depth_data(depth_frame)?;

    let mut bytes: Vec<u8> = Vec::new();
    bytes.resize(depth_length, 0);

    for y in 0..height {
        for x in 0..width {
            let dest_index = (y * width + x) as usize;
            let depth = data[dest_index] as u8;
            bytes[dest_index] = depth;
        }
    }
    astra_status_to_result(AstraStatus::Success, (width, height, depth_length, bytes))
}
