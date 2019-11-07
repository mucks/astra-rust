use super::*;
use model::Body;

pub fn get_body_frame_index(body_frame: BodyFrame) -> i32 {
    unsafe {
        let mut frame_index = 0_i32;
        sys::astra_bodyframe_get_frameindex(body_frame, &mut frame_index);
        frame_index
    }
}

pub fn start_body_stream(reader: Reader) -> Stream {
    unsafe { start_stream(reader, StreamType::Body) }
}

pub fn get_body_frame(frame: AstraFrame) -> BodyFrame {
    unsafe {
        let mut body_frame = Box::into_raw(Box::new(sys::_astra_bodyframe::default())) as BodyFrame;
        sys::astra_frame_get_bodyframe(frame, &mut body_frame);
        body_frame
    }
}

pub fn get_bodies(body_frame: BodyFrame) -> Vec<Body> {
    unsafe {
        let mut body_list = sys::_astra_body_list::default();
        sys::astra_bodyframe_body_list(body_frame, &mut body_list);
        body_list.bodies.iter().map(|b| b.into()).collect()
    }
}
