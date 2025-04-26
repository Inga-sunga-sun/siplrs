
use crate::error::DecodeError;
use crate::image_buffer::ImageBuffer;
use crate::image_file::ImageFile;
use crate::util::{read_u16, read_u32, read_u8};
use crc32fast::hash;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::io::{Cursor, Read};
use crate::image_buffer::Image;
use crate::png::png_image;
use crate::png::png_image::Header;

type Result<T> = std::result::Result<T, DecodeError>;
#[derive(Debug,Default)]
pub struct PngReader {
    header: Cursor<Vec<u8>>,
    data: Cursor<Vec<u8>>,
}


pub const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
pub const IHDR: [u8; 4] = [73, 72, 68, 82];
pub const IDAT: [u8; 4] = [73, 68, 65, 84];
pub const IEND: [u8; 4] = [73, 69, 78, 68];

impl PngReader {

    pub fn new(header:Vec<u8>, data:Vec<u8>) -> PngReader {
        PngReader {
            header: Cursor::new(header),
            data: Cursor::new(data),
        }
    }
    
    pub fn header_mut(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.header
    }
    
    pub fn data_mut(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.data
    }

    pub fn read_chunk(img: &mut ImageFile) -> Result<PngReader> {
        let _ = Self::check_signature(img)?;
        let file_length = img.length();
        let mut header = Vec::<u8>::with_capacity(13);
        let mut data = Vec::<u8>::with_capacity(file_length);

        // length -> chunk_type -> chunk_data -> crc
        loop {
            let length = Self::read_length(img)? as usize;
            let chunk_type = Self::read_chunk_type(img)?;
            let chunk_data = Self::read_chunk_data(img, length)?;
            let crc = Self::read_crc(img)?;

            let _ = Self::verify_checksum(crc, &chunk_type, &chunk_data)?;

            match chunk_type {
                // IHDR => reader.set_header(chunk_data),
                // IDAT => reader.append_data(chunk_data)?,
                IHDR => header = chunk_data,
                IDAT => data.append(&mut decompress_to_vec_zlib(&chunk_data)?),
                IEND => break,
                _    => continue,
            }
        }
        let reader = PngReader::new(header, data);
        Ok(reader)
    }
    

    fn verify_checksum(crc: u32, chunk_type: &[u8;4], buf: &Vec<u8>) -> Result<()> {
        let checksum = [&chunk_type[0..], &buf[0..]].concat();
        let checksum = hash(&checksum);

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
    
    fn read_length(img: &mut ImageFile) -> Result<usize>{
        let mut buf = [0u8;4];
        let _ = img.data().read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf) as usize)
        
    }
    
    fn read_chunk_type(img: &mut ImageFile) -> Result<[u8; 4]>{
        let mut buf = [0u8;4];
        let _ = img.data().read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_chunk_data(img: &mut ImageFile, length: usize) -> Result<Vec<u8>>{
        let mut buf: Vec<u8> = Vec::with_capacity(length);
        let _ = img.data().take(length as u64).read_to_end(&mut buf)?;
        Ok(buf)
    }

    fn read_crc(img: &mut ImageFile) -> Result<u32>{
        let mut crc = [0u8;4];
        let _ = img.data().read_exact(&mut crc)?;
        let crc = u32::from_be_bytes(crc);
        Ok(crc)
    }
    

    pub fn convert_to_image(&mut self) -> Result<Image>
    {
        let header = self.header_mut();
        let mut buf_u8 = [0u8;1];
        let mut buf_u32 = [0u8;4];

        let header = self.read_header(&mut buf_u8, &mut buf_u32)?;
        
        let mut image = Image::new(header.width(), header.height(), header.bit_depth());
        
        match image {
            Image::Gray8(ref mut img) => {
                let _ = self.read_data8(img)?;
            },
            Image::Gray16(ref mut img) => {
                let _ = self.read_data16(img)?;
            },
            _ => unimplemented!(),
        }
        Ok(image)
    }

    pub fn read_header(&mut self, buf_u8: &mut [u8;1], buf_u32: &mut [u8; 4]) -> Result<Header> {
        
        let header = self.header_mut();
        let width = read_u32(header, buf_u32)? as usize;
        let height = read_u32(header, buf_u32)? as usize;
        let bit_depth = read_u8(header, buf_u8)?;
        let color_type = read_u8(header,buf_u8)?;
        let compression_method = read_u8(header, buf_u8)?;
        let filter_method = read_u8(header, buf_u8)?;
        let interlace_method = read_u8(header, buf_u8)?;
        
        let header = Header {
            width,
            height,
            bit_depth,
            color_type,
            compression_method,
            filter_method,
            interlace_method,
        };
        
        Ok(header)
    }

    pub fn read_data16(&mut self, buf: &mut ImageBuffer<u16>) -> Result<()>
    {
        let mut buf_u8=[0u8;1];
        let mut buf_u16 =[0u8;2];
        let mut cur = self.data_mut();
        
        for h in 0..buf.height() {
            let _ = cur.read_exact(&mut buf_u8);
            for w in 0..buf.width() {
                let value = read_u16(&mut cur, &mut buf_u16)?;
                buf.push(value);
            }
        }
        Ok(())
    }

    pub fn read_data8(&mut self, buf: &mut ImageBuffer<u8>) -> Result<()>
    {
        let mut buf_u8=[0u8;1];

        let mut cur = self.data_mut();
        
        for h in 0..buf.height() {
            let _ = cur.read_exact(&mut buf_u8);
            for w in 0..buf.width() {
                let value = read_u8(&mut cur, &mut buf_u8)?;
                buf.push(value);
            }
        }
        Ok(())

    }

}



