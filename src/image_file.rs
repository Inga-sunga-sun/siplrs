use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Cursor};
use std::path::Path;
use crate::error::decode_error::DecodeError;
use crate::image_buffer::buffer::{ImageBuffer};
use crate::png::png_reader::PngReader;
use crate::util::GrayValue;

type Result<T> = std::result::Result<T, DecodeError>;
#[derive(Debug)]
pub struct ImageFile {
    length: usize,
    data: Cursor<Vec<u8>>,
    extension: ImageFormat,
}
#[derive(Debug)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Bmp,
    Tiff,
    Gif,
    Unknown,
}

impl ImageFile {
    pub fn open<P>(path:P) -> Result<ImageBuffer<GrayValue>>
    where P: AsRef<Path>
    {
        let mut file = File::open(&path)?;
        let mut image_file = Self::new(file, &path)?;
        
        match image_file.extension {
            ImageFormat::Png => {
                let mut reader = PngReader::read_chunk(&mut image_file)?;
                reader.convert_to_image()
            },
            _ => unimplemented!(),

        }
        
        
        
    }
    
    pub fn new<P>(mut file: File, path: P) -> Result<Self> 
    where P: AsRef<Path>
    {
        let length = file.metadata()?.len() as usize;
        let mut buf: Vec<u8> = Vec::with_capacity(length);
        let _ = file.read_to_end(&mut buf)?;
        let extension = ImageFormat::extension(&path);
        
        let image_file = Self {length, data: Cursor::new(buf), extension};
        
        Ok(image_file)
    }



    // pub fn data(&mut self) -> &mut Cursor<Vec<u8>> {
    pub fn data(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.data
    }

    pub fn length(&mut self) -> usize {
        self.length
    }

}

impl ImageFormat {
    pub fn extension<P>(ext: P) -> Self
    where P: AsRef<Path>
    {
        let ext = ext.as_ref().extension().and_then(OsStr::to_str);
        match ext {
            Some("png") => Self::Png,
            Some("jpg" | "jpeg") => Self::Jpeg,
            Some("bmp") => Self::Bmp,
            Some("tiff") => Self::Tiff,
            Some("gif") => Self::Gif,
            _ => Self::Unknown,
        }
    }
}
