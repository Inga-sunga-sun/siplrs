use std::io::{Cursor, Read};
use std::marker::PhantomData;
use crate::error::DecodeError;
use crate::png::{PngRead, PngReader};
use crate::util::{read_u16, read_u32, read_u8, GrayValue};
type Result<T> = std::result::Result<T, DecodeError>;

#[derive(Debug,Default)]
pub struct ImageBuffer<T>
where T: GrayValue
{
    width: usize,
    height: usize,
    data: Vec<T>
}

impl<T:GrayValue> ImageBuffer<T>
{
    
    pub fn new(data:Vec<T>) -> ImageBuffer<T> 
    where T: GrayValue
    {
        ImageBuffer{
            width:0,
            height:0,
            data
        }
    }

    
}

pub fn read_header<T: GrayValue>(reader:&PngReader) -> ImageBuffer<T>
{
    let mut cursor = Cursor::new(&reader.header);
    let mut buf_u8 = [0u8;1];
    let mut buf_u32 = [0u8;4];

    let width = read_u32(&mut cursor, &mut buf_u32) as usize;
    let height = read_u32(&mut cursor, &mut buf_u32) as usize;
    let bit_depth = read_u8(&mut cursor, &mut buf_u8);
    let color_type = read_u8(&mut cursor, &mut buf_u8);
    let compression_method = read_u8(&mut cursor, &mut buf_u8);
    let filter_method = read_u8(&mut cursor, &mut buf_u8);
    let interlace_method = read_u8(&mut cursor, &mut buf_u8);
    
    
}


