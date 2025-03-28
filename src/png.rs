use crate::error::DecodeError;
use crate::image_buffer::ImageBuffer;
use crate::image_file::ImageFile;
use crate::util::{read_u16, read_u32, read_u8, GrayValue};
use crc32fast::hash;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::io::{Cursor, Read};


type Result<T> = std::result::Result<T, DecodeError>;
#[derive(Debug,Default)]
pub struct PngReader {
    pub header: Vec<u8>,
    pub data: Vec<u8>,
}


pub const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
pub const IHDR: [u8; 4] = [73, 72, 68, 82];
pub const IDAT: [u8; 4] = [73, 68, 65, 84];
pub const IEND: [u8; 4] = [73, 69, 78, 68];

pub trait PngRead<T>
where T: GrayValue
{
    fn read_data(&self, width: usize, height: usize) -> Vec<T>;
}

impl PngReader {

    pub fn read_chunk(&mut self, img: &mut ImageFile) -> Result<()> {
        let _ = Self::check_signature(img)?;

        // length -> chunk_type -> chunk_data -> crc
        loop {
            let length = Self::length(img)?;
            let chunk_type = Self::chunk_type(img)?;
            let chunk_data = Self::chunk_data(img, length)?;
            let crc = Self::crc(img)?;

            let _ = Self::verify_checksum(crc, &chunk_type, &chunk_data)?;

            match chunk_type {
                IHDR => self.set_header(chunk_data),
                IDAT => self.set_data(chunk_data)?,
                IEND => break,
                _    => continue,
            }
        }
        Ok(())
    }
    

    

    fn verify_checksum(crc: u32, chunk_type: &[u8;4], buf: &Vec<u8>) -> Result<()> {
        let checksum = [&chunk_type[0..], &buf[0..]].concat();
        let checksum = hash(& checksum);

        if crc != checksum {
            //Err(io::Error::new(io::ErrorKind::Other, "checksum mismatch"))
            Err(DecodeError::ChecksumError)
        } else {
            Ok(())
        }

    }

    fn check_signature(img: &mut ImageFile) -> Result<()>
    {
        let mut signature= [0u8; 8];
        let _ = img.data().read_exact(&mut signature)?;

        if signature != PNG_SIGNATURE {
            //Err(io::Error::new(io::ErrorKind::Other, "signature mismatch"))
            Err(DecodeError::SignatureError)
        } else {
            Ok(())
        }

    }

    fn length(img: &mut ImageFile) -> Result<usize>{
        let mut length = [0u8;4];
        let _ = img.data().read_exact(&mut length)?;
        let length = u32::from_be_bytes(length);
        Ok(length as usize)
    }

    fn chunk_type(img: &mut ImageFile) -> Result<[u8; 4]>{
        let mut chunk_type = [0u8;4];
        let _ = img.data().read_exact(&mut chunk_type)?;
        Ok(chunk_type)
    }

    fn chunk_data(img: &mut ImageFile, length: usize) -> Result<Vec<u8>>{
        let mut buf: Vec<u8> = Vec::with_capacity(length);
        let _ = img.data().take(length as u64).read_to_end(&mut buf)?;
        Ok(buf)
    }

    fn crc(img: &mut ImageFile) -> Result<u32>{
        let mut crc = [0u8;4];
        let _ = img.data().read_exact(&mut crc)?;
        let crc = u32::from_be_bytes(crc);
        Ok(crc)
    }

    pub fn set_data(&mut self, chunk_data: Vec<u8>) -> Result<()>{
        chunk_data.as_slice();
        self.data = decompress_to_vec_zlib(&chunk_data)?;
        Ok(())
    }
    pub fn set_header(&mut self, chunk_data: Vec<u8>){
        self.header = chunk_data
    }
    pub fn read_data16(&self, width:usize, height: usize) -> Vec<u16>
    {
        let mut buf_u8=[0u8;1];
        let mut buf_u16 =[0u8;2];
        let mut cur = Cursor::new(&self.data);

        let mut v = Vec::new();
        for h in 0..height {
            let _ = cur.read_exact(&mut buf_u8);
            for w in 0..width {
                let value = read_u16(&mut cur, &mut buf_u16);
                v.push(value);
            }
        }
        v

    }

    pub fn read_data8(&self, width:usize, height: usize) -> Vec<u8>
    {
        let mut buf_u8=[0u8;1];

        let mut cur = Cursor::new(&self.data);

        let mut v = Vec::new();
        for h in 0..height {
            let _ = cur.read_exact(&mut buf_u8);
            for w in 0..width {
                let value = read_u8(&mut cur, &mut buf_u8);
                v.push(value);
            }
        }
        v

    }
    
}



