use super::stream::start_stream;
use super::types::*;
use crate::util::astra_status_to_result;
use model::{Body, Result, StreamType};

pub fn get_body_frame_index(body_frame: BodyFrame) -> i32 {
    unsafe {
        let mut frame_index = 0_i32;
        sys::astra_bodyframe_get_frameindex(body_frame, &mut frame_index);
        frame_index
    }
}

pub fn start_body_stream(reader: Reader) -> Result<Stream> {
    start_stream(reader, StreamType::Body)
}

pub fn get_body_frame(frame: AstraFrame) -> Result<BodyFrame> {
    unsafe {
        let mut body_frame: BodyFrame = std::ptr::null_mut();
        let status = sys::astra_frame_get_bodyframe(frame, &mut body_frame);
        astra_status_to_result(status.into(), body_frame)
    }
}

pub fn get_bodies(body_frame: BodyFrame) -> Vec<Body> {
    unsafe {
        let mut body_list = sys::astra_body_list_t::default();
        sys::astra_bodyframe_body_list(body_frame, &mut body_list);
        body_list.bodies.iter().map(|b| b.into()).collect()
    }
}
