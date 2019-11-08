#[derive(Debug, Clone, Copy, Default)]
pub struct ColorMeta {
    pub width: u32,
    pub height: u32,
    pub frame_index: i32,
    pub byte_length: usize,
}
