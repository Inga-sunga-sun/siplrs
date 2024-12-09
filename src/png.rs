use std::io::Read;
use crate::image_reader::ImageReader;
use crc32fast::hash;

#[derive(Debug,Default)]
pub struct PngReader {
    width: u32,
    height: u32,
    bit_depth: u8,
    data: Vec<u8>,
}
pub const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
pub const IHDR: [u8; 4] = [0x73, 0x72, 0x68, 0x82];
pub const IDAT: [u8; 4] = [0x49, 0x44, 0x41, 0x54];
pub const IEND: [u8; 4] = [0x49, 0x45, 0x4E, 0x44];

impl PngReader {
    pub fn new(img: &ImageReader)  {

    }



    pub fn read_chunk(img: &mut ImageReader) {
        let mut signature= [0u8; 8];
        let mut length = [0u8;4];
        let mut chunk_type = [0u8;4];
        let mut crc = [0u8;4];

        let _ = img.data().read_exact(&mut signature);
        let _ = img.data().read_exact(&mut length);
        let _ = img.data().read_exact(&mut chunk_type);

        let length = u32::from_be_bytes(length);

        let mut buf: Vec<u8> = Vec::with_capacity(length as usize);
        let _ = img.data().take(length as u64).read_to_end(&mut buf);



        let _ = img.data().read_exact(&mut crc);

        let checksum = [&chunk_type[0..], &buf[0..]].concat();
        let crc32 = u32::from_be_bytes(crc);
        println!("{}", &crc32);
        let checksum = hash(& checksum);
        println!("{}", &checksum);

        println!("a");
    }
}
