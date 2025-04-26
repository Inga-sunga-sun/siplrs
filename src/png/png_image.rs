use std::io::Cursor;
use crate::image_buffer::{Image, ImageBuffer};
use crate::png::png_reader::PngReader;
use crate::util::{read_u32, read_u8, GrayValue, PixelArray};

pub struct Header {
    pub width: usize,
    pub height: usize,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,

}

impl Header {
   
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn bit_depth(&self) -> u8 {
        self.bit_depth
    }
    pub fn color_type(&self) -> u8 {
        self.color_type
    }

}