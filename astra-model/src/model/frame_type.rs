#[derive(Debug)]
pub enum FrameType {
    Frame,
    Body,
    ImageFrame(ImageFrameType),
    Color,
    MaskedColor,
    Depth,
}

#[derive(Debug)]
pub enum ImageFrameType {
    Color,
    MaskedColor,
    Depth,
}
