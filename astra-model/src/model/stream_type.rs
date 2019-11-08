#[derive(Hash, PartialEq, Clone, Copy, Eq, Debug)]
pub enum StreamType {
    Body,
    Color,
    MaskedColor,
}
