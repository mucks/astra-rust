mod body;
mod color;
mod frame;
mod masked_color;
mod reader;
mod stream;
mod stream_set;
mod types;
pub use body::*;
pub use color::*;
pub use frame::*;
pub use masked_color::*;
pub use reader::*;
pub use stream::*;
pub use stream_set::*;
pub use types::*;

pub fn update() {
    unsafe {
        sys::astra_update();
    }
}

pub fn init() {
    unsafe {
        sys::astra_initialize();
    }
}

pub fn terminate() {
    unsafe {
        sys::astra_terminate();
    }
}
