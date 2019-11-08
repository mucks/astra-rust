use super::stream::start_stream;
use super::types::*;
use model::StreamType;

pub fn start_masked_color_stream(reader: Reader) -> Stream {
    start_stream(reader, StreamType::MaskedColor)
}
pub unsafe fn get_masked_color_frame(frame: AstraFrame) -> ImageFrame {
    unsafe {
        let mut masked_color_frame =
            Box::into_raw(Box::new(sys::_astra_imageframe::default())) as MaskedColorFrame;
        sys::astra_frame_get_colorframe(frame, &mut masked_color_frame);
        masked_color_frame
    }
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
