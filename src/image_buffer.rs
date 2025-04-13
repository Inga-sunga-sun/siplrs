use std::io::{Cursor, Read};
use std::marker::PhantomData;
use crate::error::DecodeError;
use crate::util::{read_u16, read_u32, read_u8, GrayValue};
type Result<T> = std::result::Result<T, DecodeError>;

#[derive(Debug,Default)]
pub struct ImageBuffer<T:GrayValue>
{
    width: usize,
    height: usize,
    data: Vec<T>
}


impl<T:GrayValue> ImageBuffer<T> {
    pub fn new(width: usize, height: usize) -> ImageBuffer<T> {
        ImageBuffer {
            width,
            height,
            data: Vec::<T>::with_capacity(width * height)
        }
    }
    
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

pub enum Image {
    Gray8(ImageBuffer<u8>),
    Gray16(ImageBuffer<u16>),
}

impl Image {
    pub fn new(width:usize, height: usize, bit_depth:u8) -> Self {
        match bit_depth {
            8 => Image::Gray8(ImageBuffer::<u8>::new(width, height)),
            16 => Image::Gray16(ImageBuffer::<u16>::new(width, height)),
            _ => unimplemented!(),
        }
    }
    

}



