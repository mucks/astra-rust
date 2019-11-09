use super::img::*;
use super::types::*;
use crate::util::astra_status_to_result;
use model::Result;

//ByteArray::new().write().as_mut_ptr()
pub fn get_masked_color_bytes(
    masked_color_frame: MaskedColorFrame,
) -> Result<(u32, u32, usize, Vec<u8>)> {
    let mut rgba_pixels = Vec::new();
    let (width, height) = get_img_frame_dimensions(masked_color_frame)?;
    let byte_length = get_img_frame_byte_length(masked_color_frame)?;
    rgba_pixels.resize(
        (width * height) as usize,
        sys::astra_rgba_pixel_t {
            r: 0,
            g: 0,
            b: 0,
            alpha: 0,
        },
    );
    let status = unsafe {
        sys::astra_maskedcolorframe_copy_data(masked_color_frame, rgba_pixels.as_mut_ptr())
    };
    let mut bytes = Vec::new();
    bytes.resize(byte_length, 0);

    for y in 0..height {
        for x in 0..width {
            let src_index = (y * width + x) as usize;
            let dest_index = src_index * 4;
            let pixel = rgba_pixels[src_index];

            if pixel.alpha != 0 {
                bytes[dest_index] = pixel.r;
                bytes[dest_index + 1] = pixel.g;
                bytes[dest_index + 2] = pixel.b;
                bytes[dest_index + 3] = pixel.alpha;
            }
        }
    }

    astra_status_to_result(status.into(), (width, height, byte_length, bytes))
}
