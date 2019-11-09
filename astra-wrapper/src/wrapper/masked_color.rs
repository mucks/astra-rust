use super::types::*;
use crate::util::astra_status_to_result;
use model::Result;

pub fn get_masked_color_frame(frame: AstraFrame) -> Result<MaskedColorFrame> {
    unsafe {
        let mut masked_color_frame =
            Box::into_raw(Box::new(sys::_astra_imageframe::default())) as MaskedColorFrame;
        let status = sys::astra_frame_get_colorframe(frame, &mut masked_color_frame);
        astra_status_to_result(status.into(), masked_color_frame)
    }
}

pub fn get_masked_color_frame_index(masked_color_frame: MaskedColorFrame) -> Result<i32> {
    unsafe {
        let mut frame_index = 0_i32;
        let status =
            sys::astra_maskedcolorframe_get_frameindex(masked_color_frame, &mut frame_index);
        astra_status_to_result(status.into(), frame_index)
    }
}

pub unsafe fn get_masked_color_frame_byte_length(color_frame: MaskedColorFrame) -> Result<usize> {
    let mut byte_length = 0;
    let status = sys::astra_maskedcolorframe_get_data_byte_length(color_frame, &mut byte_length);
    astra_status_to_result(status.into(), byte_length as usize)
}

pub unsafe fn get_masked_color_frame_dimensions(
    masked_color_frame: MaskedColorFrame,
) -> Result<(u32, u32)> {
    let mut metadata = sys::astra_image_metadata_t::default();
    let status = sys::astra_maskedcolorframe_get_metadata(masked_color_frame, &mut metadata);
    astra_status_to_result(status.into(), (metadata.width, metadata.height))
}

//ByteArray::new().write().as_mut_ptr()
pub fn get_masked_color_bytes(masked_color_frame: MaskedColorFrame) -> Result<Vec<u8>> {
    unsafe {
        let (width, height) = get_masked_color_frame_dimensions(masked_color_frame)?;
        let mut rgba_pixels = Vec::new();
        let byte_length = get_masked_color_frame_byte_length(masked_color_frame)?;
        rgba_pixels.resize(
            (width * height) as usize,
            sys::astra_rgba_pixel_t {
                r: 0,
                g: 0,
                b: 0,
                alpha: 0,
            },
        );
        let status =
            sys::astra_maskedcolorframe_copy_data(masked_color_frame, rgba_pixels.as_mut_ptr());
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
        astra_status_to_result(status.into(), bytes)
    }
}
