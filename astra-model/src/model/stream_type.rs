#[derive(Hash, PartialEq, Clone, Copy, Eq, Debug)]
pub enum StreamType {
    Body,
    Color,
    MaskedColor,
    Depth,
    Infrared,
}

impl StreamType {
    #[cfg(feature = "godot")]
    pub fn godot_image_format(&self) -> i64 {
        match self {
            StreamType::Color => 4,
            StreamType::MaskedColor => 5,
            StreamType::Depth => 0,
            _ => 0,
        }
    }
}
