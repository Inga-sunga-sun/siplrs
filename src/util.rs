use std::io;
use std::io::{Cursor, Read};
use num_traits::{One, Zero};

pub(crate) type GrayValue = u16;



pub fn read_u8(cursor:&mut Cursor<Vec<u8>>, buf: &mut [u8;1]) -> io::Result<u8> {
     
    let _ = cursor.read_exact(buf)?;
    let value = u8::from_be_bytes(*buf);
    Ok(value)
}

pub fn read_u16(cursor:&mut Cursor<Vec<u8>>, buf: &mut [u8;2]) -> io::Result<u16> {
    let _ = cursor.read_exact(buf)?;
    let value = u16::from_be_bytes(*buf);
    Ok(value)
}

pub fn read_u32(cursor:&mut Cursor<Vec<u8>>, buf: &mut [u8;4]) -> io::Result<u32> {
    let _ = cursor.read_exact(buf)?;
    let value = u32::from_be_bytes(*buf);
    Ok(value)
}

