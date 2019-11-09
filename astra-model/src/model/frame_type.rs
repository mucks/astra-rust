#[derive(Debug, Clone)]
pub enum FrameType {
    Frame,
    Body,
    ImageFrame(ImageFrameType),
    Color,
    MaskedColor,
    Depth,
}

#[derive(Debug, Clone)]
pub enum ImageFrameType {
    Color,
    MaskedColor,
    Depth,
}
