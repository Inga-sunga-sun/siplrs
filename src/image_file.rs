use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Cursor;
use std::path::Path;

pub struct ImageFile {
    data: Cursor<Vec<u8>>,
}

impl ImageFile {
    pub fn open<P>(path:P) -> io::Result<Self>
    where P: AsRef<Path>
    {
        let mut file = File::open(path)?;
        let mut buf: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut buf)?;
        let data  = Cursor::new(buf);
        Ok(
            Self {data}
        )
    }



    // pub fn data(&mut self) -> &mut Cursor<Vec<u8>> {
    pub fn data(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.data
    }

}