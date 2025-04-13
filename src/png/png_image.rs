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

}