#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    Frame,
    Body,
    Color,
    MaskedColor,
    Depth,
}
