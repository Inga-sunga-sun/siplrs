
use std::marker::PhantomData;
use crate::error::decode_error::DecodeError;
use crate::image_file::ImageFormat;
use crate::png::png_image::Header;
use crate::util::{read_u16, read_u32, read_u8, GrayValue};
type Result<T> = std::result::Result<T, DecodeError>;

#[derive(Debug,Default, PartialEq, Clone)]
pub struct ImageBuffer<GrayValue>
{
    width: usize,
    height: usize,
    bit_depth: u8,
    data: Vec<GrayValue>,
}


impl ImageBuffer<GrayValue> {
    pub fn new(header: &Header) -> ImageBuffer<GrayValue> {
        let width = header.width;
        let height = header.height;
        let bit_depth = header.bit_depth;
        
        ImageBuffer {
            width,
            height,
            bit_depth,
            data: Vec::with_capacity(width * height),
        }
    }
    
    pub fn push(&mut self, value: GrayValue) {
        self.data.push(value);
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn data(&mut self) -> &mut [GrayValue] {
        &mut self.data
    }
}
// #[derive(Debug, PartialEq, Clone)]
// pub enum Image {
//     Gray8(ImageBuffer<u8>),
//     Gray16(ImageBuffer<u16>),
// }
// 
// impl Image {
//     pub fn new(width:usize, height: usize, bit_depth:u8) -> Self {
//         match bit_depth {
//             8 => Image::Gray8(ImageBuffer::<u8>::new(width, height)),
//             16 => Image::Gray16(ImageBuffer::<u16>::new(width, height)),
//             _ => unimplemented!(),
//         }
//     }
// }



