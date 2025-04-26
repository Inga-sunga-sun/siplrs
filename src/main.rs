use std::io::{Cursor, Read};
use std::path::Path;
use siplrs::image_buffer::{Image, ImageBuffer};
use siplrs::image_file;
use siplrs::image_file::ImageFile;
use siplrs::png::png_reader::PngReader;
use siplrs::util::read_u16;


fn main() {
    // let path_in = Path::new(r"D:\006_Source\Assets\5x5_u16.png");
    let path_in = Path::new(r"C:\Users\USER\Documents\006_Source\Assets\5x5_u16.png");
    let mut img = ImageFile::open(path_in).unwrap();

    println!("here");

}
