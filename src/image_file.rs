use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Cursor;
use std::path::Path;

pub struct ImageFile {
    length: usize,
    data: Cursor<Vec<u8>>,
}

impl ImageFile {
    pub fn open<P>(path:P) -> io::Result<Self>
    where P: AsRef<Path>
    {
        let mut file = File::open(path)?;

        let length = file.metadata()?.len() as usize;
        let mut buf: Vec<u8> = Vec::with_capacity(length);

        let _ = file.read_to_end(&mut buf)?;
        let data  = Cursor::new(buf);
        Ok(
            Self {length, data}
        )
    }



    // pub fn data(&mut self) -> &mut Cursor<Vec<u8>> {
    pub fn data(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.data
    }

    pub fn length(&mut self) -> usize {
        self.length
    }

}