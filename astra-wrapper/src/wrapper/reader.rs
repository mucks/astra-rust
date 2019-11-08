use super::types::{Reader, StreamSet};
use crate::util::astra_status_to_result;
use model::Result;

pub fn get_reader(sensor: StreamSet) -> Result<Reader> {
    unsafe {
        let mut reader = Box::into_raw(Box::new(sys::_astra_reader::default())) as Reader;
        let status = sys::astra_reader_create(sensor, &mut reader);
        astra_status_to_result(status.into(), reader)
    }
}

pub fn close_reader(reader: &mut Reader) -> Result<()> {
    unsafe {
        let status = sys::astra_reader_destroy(reader);
        astra_status_to_result(status.into(), ())
    }
}
