use std::io::{Cursor, Read};

pub trait GrayValue {}
impl GrayValue for u8 {}
impl GrayValue for u16 {}

#[derive(Debug)]
pub enum PixelArray {
    Gray8(Vec<u8>),
    Gray16(Vec<u16>),
}

impl PixelArray {
    pub fn new(width: usize, height: usize, bit_depth: u8) -> Self {
        match bit_depth {
            8 => PixelArray::Gray8(Vec::<u8>::with_capacity(width * height)),
            16 => PixelArray::Gray16(Vec::<u16>::with_capacity(width * height)),
            _ => unimplemented!(),
        }
    }
    

}






pub fn read_u8(cursor:&mut Cursor<&Vec<u8>>, buf: &mut [u8; 1]) -> u8 {
    let _ = cursor.read_exact(buf);
    let value = u8::from_be_bytes(*buf);
    value
}

pub fn read_u16(cursor:&mut Cursor<&Vec<u8>>, buf:&mut [u8; 2]) -> u16 {
    let _ = cursor.read_exact(buf);
    let value = u16::from_be_bytes(*buf);
    value
}

pub fn read_u32(cursor:&mut Cursor<&Vec<u8>>, buf:&mut [u8; 4]) -> u32 {
    let _ = cursor.read_exact(buf);
    let value = u32::from_be_bytes(*buf);
    value
}

