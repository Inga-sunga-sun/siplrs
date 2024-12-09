
use std::path::Path;
use siplrs::image_reader;
use siplrs::image_reader::ImageReader;
use siplrs::png::PngReader;

fn main() {
    let path_in = Path::new(r"D:\006_Source\Assets\5x5_u16.png");
    let mut img = ImageReader::open(path_in).unwrap();
    PngReader::read_chunk(&mut img);

    println!("here");

}
