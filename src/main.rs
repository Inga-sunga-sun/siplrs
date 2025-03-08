use std::io::{Cursor, Read};
use std::path::Path;
use siplrs::image_reader;
use siplrs::image_reader::ImageReader;
use siplrs::png::PngReader;


fn main() {
    let path_in = Path::new(r"D:\006_Source\Assets\5x5_u16.png");
    let mut img = ImageReader::open(path_in).unwrap();
    let mut pngreader = PngReader::default();
    let _ = pngreader.read_chunk(&mut img).unwrap();
    println!("{:?}", pngreader.data);
    let vec:Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let length = pngreader.data.len();

    let mut buf_u8=[0u8;1];
    let mut buf_u16=[0u8;2];
    let mut cur = Cursor::new(pngreader.data);
    let mut v = Vec::new();
    for h in 0..5 {
        let _ = cur.read_exact(&mut buf_u8);
        for w in 0..5 {
            let _ = cur.read_exact(&mut buf_u16);
            let value = u16::from_be_bytes(buf_u16);
            v.push((value));
        }
    }

    println!("here");

}
