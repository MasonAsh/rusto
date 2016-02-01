extern crate byteorder;

use std::mem;

pub struct BufferData {
    pub bytes: Vec<u8>
}

impl BufferData {
    pub fn new() -> BufferData {
        BufferData {
            bytes: Vec::new(),
        }
    }

    pub fn new_initialized<T>(data: Vec<T>) -> BufferData {
        let mut result = BufferData::new();
        result.add_data(data);
        result
    }

    pub fn new_zero_initialized(size: usize) -> BufferData {
        let mut bytes = Vec::with_capacity(size);

        for _ in 0..size {
            bytes.push(0);
        }

        BufferData {
            bytes: bytes
        }
    }

    fn convert_to_bytes<T>(data: Vec<T>) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        for datum in data.iter() {
            let view = datum as *const _ as *const u8;
            for i in 0..mem::size_of::<T>() as isize {
                unsafe {
                    result.push(*view.offset(i));
                }
            }
        }

        result
    }

    pub fn add_data<T>(&mut self, data: Vec<T>) {
        let bytes = BufferData::convert_to_bytes(data);

        self.bytes.extend(bytes);
    }

    pub fn update_region<T>(&mut self, start: usize, data: Vec<T>) {
        let inserting_bytes = BufferData::convert_to_bytes(data);

        for (i, byte) in inserting_bytes.iter().enumerate() {
            self.bytes[start + i] = *byte;
        }
    }
}
