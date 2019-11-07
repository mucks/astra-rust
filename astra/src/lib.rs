#![warn(clippy::all)]

#[macro_use]
extern crate num_derive;
extern crate astra_sys as sys;

use std::ffi::CString;

mod model;
pub use model::*;

mod body;
pub use body::*;

mod err;
mod frame;
mod sensor;

pub type Reader = sys::astra_reader_t;
pub type Stream = sys::astra_streamconnection_t;
pub type StreamSet = sys::astra_streamsetconnection_t;
pub type AstraFrame = sys::astra_reader_frame_t;
pub type ImageFrame = sys::astra_imageframe_t;
pub type ColorFrame = sys::astra_colorframe_t;
pub type BodyFrame = sys::astra_bodyframe_t;
pub type MaskedColorFrame = sys::astra_maskedcolorframe_t;

#[derive(Hash, PartialEq, Clone, Copy, Eq, Debug)]
pub enum StreamType {
    Body,
    Color,
    MaskedColor,
}

#[derive(Debug)]
pub enum FrameType {
    Frame,
    Body,
    Color,
    MaskedColor,
}

#[cfg(test)]
mod tests;

pub fn init() {
    unsafe {
        sys::astra_initialize();
    }
}

pub fn terminate() {
    unsafe {
        sys::astra_terminate();
    }
}

pub fn get_sensor() -> StreamSet {
    unsafe {
        let mut sensor =
            Box::into_raw(Box::new(sys::_astra_streamsetconnection { _unused: [] })) as StreamSet;
        let path = CString::new("device/default").unwrap();
        sys::astra_streamset_open(path.as_ptr(), &mut sensor);
        sensor
    }
}

pub fn get_reader(sensor: StreamSet) -> Reader {
    unsafe {
        let mut reader = Box::into_raw(Box::new(sys::_astra_reader::default())) as Reader;
        sys::astra_reader_create(sensor, &mut reader);
        reader
    }
}

fn start_stream(reader: Reader, stream_type: StreamType) -> Stream {
    use self::StreamType::*;
    unsafe {
        let mut stream = Box::into_raw(Box::new(sys::_astra_streamconnection::default())) as Stream;
        match stream_type {
            Body => sys::astra_reader_get_bodystream(reader, &mut stream),
            Color => sys::astra_reader_get_colorstream(reader, &mut stream),
            MaskedColor => sys::astra_reader_get_maskedcolorstream(reader, &mut stream),
        };
        sys::astra_stream_start(stream);
        stream
    }
}
pub fn start_masked_color_stream(reader: Reader) -> Stream {
    start_stream(reader, StreamType::MaskedColor)
}
pub fn start_color_stream(reader: Reader) -> Stream {
    start_stream(reader, StreamType::Color)
}

pub fn stop_stream(stream: Stream) {
    unsafe {
        sys::astra_stream_stop(stream);
    }
}

pub fn update() {
    unsafe {
        sys::astra_update();
    }
}

pub fn close_frame(frame: &mut AstraFrame) {
    unsafe {
        sys::astra_reader_close_frame(frame);
    }
}

pub fn get_frame(reader: Reader) -> Option<AstraFrame> {
    unsafe {
        let mut frame = Box::into_raw(Box::new(sys::_astra_reader_frame::default())) as AstraFrame;
        let rc = sys::astra_reader_open_frame(reader, 0, &mut frame);

        if rc == sys::astra_status_t_ASTRA_STATUS_SUCCESS {
            Some(frame)
        } else {
            None
        }
    }
}

fn get_image_frame(frame: AstraFrame, masked: bool) -> ImageFrame {
    unsafe {
        let mut image_frame =
            Box::into_raw(Box::new(sys::_astra_imageframe::default())) as ImageFrame;
        if masked {
            sys::astra_frame_get_maskedcolorframe(frame, &mut image_frame);
        } else {
            sys::astra_frame_get_colorframe(frame, &mut image_frame);
        }
        image_frame
    }
}

pub fn get_color_frame(frame: AstraFrame) -> ColorFrame {
    unsafe {
        let mut color_frame =
            Box::into_raw(Box::new(sys::_astra_imageframe::default())) as ColorFrame;
        sys::astra_frame_get_colorframe(frame, &mut color_frame);
        color_frame
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
pub unsafe fn get_color_byte_array(color_frame: sys::astra_colorframe_t, ptr: *mut u8) {
    sys::astra_colorframe_copy_data(color_frame, ptr);
}

pub fn close_reader(reader: &mut Reader) {
    unsafe {
        sys::astra_reader_destroy(reader);
    }
}

fn close_stream_set(stream_set: &mut StreamSet) {
    unsafe {
        sys::astra_streamset_close(stream_set);
    }
}

pub unsafe fn get_masked_color_frame(frame: AstraFrame) -> ImageFrame {
    get_image_frame(frame, true)
}

pub unsafe fn get_masked_color_frame_index(
    masked_color_frame: sys::astra_maskedcolorframe_t,
) -> i32 {
    let mut frame_index = 0_i32;
    sys::astra_maskedcolorframe_get_frameindex(masked_color_frame, &mut frame_index);
    frame_index
}

pub unsafe fn get_masked_color_frame_byte_length(color_frame: sys::astra_colorframe_t) -> usize {
    let mut byte_length = 0;
    sys::astra_maskedcolorframe_get_data_byte_length(color_frame, &mut byte_length);
    byte_length as usize
}

pub unsafe fn get_masked_color_frame_dimensions(
    color_frame: sys::astra_colorframe_t,
) -> (u32, u32) {
    let mut metadata = sys::astra_image_metadata_t::default();
    sys::astra_maskedcolorframe_get_metadata(color_frame, &mut metadata);
    (metadata.width, metadata.height)
}

pub fn get_color_bytes(color_frame: sys::astra_colorframe_t, byte_length: u32) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    data.resize(byte_length as usize, 0);
    unsafe {
        sys::astra_colorframe_copy_data(color_frame, data.as_mut_ptr());
    }
    data
}

//ByteArray::new().write().as_mut_ptr()
pub unsafe fn get_masked_color_byte_array(color_frame: sys::astra_colorframe_t, ptr: *mut u8) {
    let (width, height) = get_masked_color_frame_dimensions(color_frame);
    let mut rgba_pixels = Vec::new();
    let byte_length = get_masked_color_frame_byte_length(color_frame);
    rgba_pixels.resize(
        (width * height) as usize,
        sys::astra_rgba_pixel_t {
            r: 0,
            g: 0,
            b: 0,
            alpha: 0,
        },
    );
    sys::astra_maskedcolorframe_copy_data(color_frame, rgba_pixels.as_mut_ptr());
    let mut bytes = Vec::new();
    bytes.resize(byte_length, 0);
    for i in 0..rgba_pixels.len() {
        let pixel = rgba_pixels[i];
        if pixel.alpha == 0 {
            bytes[i] = pixel.r;
            bytes[i + 1] = pixel.g;
            bytes[i + 2] = pixel.b;
            bytes[i + 3] = pixel.alpha;
        } else {
            //implement depth
        }
    }
    std::ptr::copy(bytes.as_ptr(), ptr, byte_length);
}
