use crate::util::GrayValue;

pub struct PngImage<T: GrayValue> {
    pub width: usize,
    pub height: usize,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
    pub data: Vec<T>,

}