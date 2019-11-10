#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    Frame,
    Body,
    ImageFrame(ImageFrameType),
    Color,
    MaskedColor,
    Depth,
}

#[derive(Debug, Clone, Copy)]
pub enum ImageFrameType {
    Color,
    MaskedColor,
    Depth,
}
