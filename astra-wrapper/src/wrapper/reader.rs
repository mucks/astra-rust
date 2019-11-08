use super::types::{Reader, StreamSet};

pub fn get_reader(sensor: StreamSet) -> Reader {
    unsafe {
        let mut reader = Box::into_raw(Box::new(sys::_astra_reader::default())) as Reader;
        sys::astra_reader_create(sensor, &mut reader);
        reader
    }
}

pub fn close_reader(reader: &mut Reader) {
    unsafe {
        sys::astra_reader_destroy(reader);
    }
}
