use std::io::{Cursor, Read};
use std::path::Path;
use siplrs::image_buffer::ImageBuffer;
use siplrs::image_file;
use siplrs::image_file::ImageFile;
use siplrs::png::{PngReader};
use siplrs::util::read_u16;


fn main() {
    let path_in = Path::new(r"C:\Users\USER\Documents\006_Source\Assets\5x5_u16.png");
    let mut img = ImageFile::open(path_in).unwrap();
    let mut pngreader = PngReader::default();
    let _ = pngreader.read_chunk(&mut img).unwrap();
    println!("{:?}", pngreader.data);
    let vec:Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let vec16 = Vec::<u32>::new();
    let length = pngreader.data.len();
    

    println!("here");

}
