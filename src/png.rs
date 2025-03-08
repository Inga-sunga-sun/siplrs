
use std::io;
use std::io::{Cursor, Read};
use crate::image_reader::ImageReader;
use crate::image_buffer::ImageBuffer;
use crc32fast::hash;
use miniz_oxide::inflate::{decompress_to_vec, decompress_to_vec_zlib};
use crate::error::DecodeError;

type Result<T> = std::result::Result<T, DecodeError>;
#[derive(Debug,Default)]
pub struct PngReader {
    header: Vec<u8>,
    pub data: Vec<u8>,
}


pub const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
pub const IHDR: [u8; 4] = [73, 72, 68, 82];
pub const IDAT: [u8; 4] = [73, 68, 65, 84];
pub const IEND: [u8; 4] = [73, 69, 78, 68];



impl PngReader {
    pub fn read_chunk(&mut self, img: &mut ImageReader) -> Result<()> {
        let _ = Self::check_signature(img)?;

        // length -> chunk_type -> chunk_data -> crc
        loop {
            let length = Self::length(img)?;
            let chunk_type = Self::chunk_type(img)?;
            let mut chunk_data = Self::chunk_data(img, length)?;
            let crc = Self::crc(img)?;

            let _ = Self::detect_error(crc, &chunk_type, &chunk_data)?;

            match chunk_type {
                IHDR => self.set_header(chunk_data),
                IDAT => self.set_data(chunk_data)?,
                IEND => break,
                _    => continue,
            }
        }
        Ok(())
    }




    pub fn read_header(&mut self, buf: &Vec<u8>) {
        let mut cursor = Cursor::new(buf);

        let mut width = [0u8;4];
        let mut height = [0u8;4];
        let mut bit_depth = [0u8;1];
        let mut color_type = [0u8;1];
        let mut compression_method = [0u8;1];
        let mut filter_method = [0u8;1];
        let mut interlace_method = [0u8;1];

        let _ = cursor.read_exact(&mut width);
        let _ = cursor.read_exact(&mut height);
        let _ = cursor.read_exact(&mut bit_depth);
        let _ = cursor.read_exact(&mut color_type);
        let _ = cursor.read_exact(&mut compression_method);
        let _ = cursor.read_exact(&mut filter_method);
        let _ = cursor.read_exact(&mut interlace_method);

        // self.width = u32::from_be_bytes(width) as usize;
        // self.height = u32::from_be_bytes(height) as usize;
        // self.bit_depth = u8::from_be_bytes(bit_depth);
        // self.color_type = u8::from_be_bytes(color_type);
        // self.compression_method = u8::from_be_bytes(compression_method);
        // self.filter_method = u8::from_be_bytes(filter_method);
        // self.interlace_method = u8::from_be_bytes(interlace_method);


    }

    fn detect_error(crc: u32, chunk_type: &[u8;4], buf: &Vec<u8>) -> Result<()> {
        let checksum = [&chunk_type[0..], &buf[0..]].concat();
        let checksum = hash(& checksum);

        if crc != checksum {
            //Err(io::Error::new(io::ErrorKind::Other, "checksum mismatch"))
            Err(DecodeError::ChecksumError)
        } else {
            Ok(())
        }

    }

    fn check_signature(img: &mut ImageReader) -> Result<()>
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

    fn length(img: &mut ImageReader) -> Result<usize>{
        let mut length = [0u8;4];
        let _ = img.data().read_exact(&mut length)?;
        let length = u32::from_be_bytes(length);
        Ok(length as usize)
    }

    fn chunk_type(img: &mut ImageReader) -> Result<[u8; 4]>{
        let mut chunk_type = [0u8;4];
        let _ = img.data().read_exact(&mut chunk_type)?;
        Ok(chunk_type)
    }

    fn chunk_data(img: &mut ImageReader, length: usize) -> Result<Vec<u8>>{
        let mut buf: Vec<u8> = Vec::with_capacity(length);
        let _ = img.data().take(length as u64).read_to_end(&mut buf)?;
        Ok(buf)
    }

    fn crc(img: &mut ImageReader) -> Result<u32>{
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

}

