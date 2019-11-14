mod body;
mod depth;
mod frame;
mod img;
mod reader;
mod stream;
mod stream_set;
mod types;
pub use body::*;
pub use depth::*;
pub use frame::*;
pub use img::*;
pub use reader::*;
pub use stream::*;
pub use stream_set::*;
pub use types::*;

use crate::util::astra_status_to_result;
use model::Result;

pub fn update() -> Result<()> {
    unsafe { astra_status_to_result(sys::astra_update().into(), ()) }
}

pub fn init() -> Result<()> {
    unsafe { astra_status_to_result(sys::astra_initialize().into(), ()) }
}

pub fn terminate() -> Result<()> {
    unsafe { astra_status_to_result(sys::astra_terminate().into(), ()) }
}
