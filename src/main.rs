use std::io::{Cursor, Read};
use std::path::Path;
use siplrs::image_buffer::{Image, ImageBuffer};
use siplrs::image_file;
use siplrs::image_file::ImageFile;
use siplrs::png::png_reader::PngReader;
use siplrs::util::read_u16;


fn main() {
    let path_in = Path::new(r"D:\006_Source\Assets\5x5_u16.png");
    let mut img = ImageFile::open(path_in).unwrap();
    let mut pngreader = PngReader::read_chunk(&mut img).unwrap();
    let vec:Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let a = pngreader.read_header();
    println!("here");

}
