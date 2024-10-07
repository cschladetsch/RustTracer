use std::fs::File;
use std::io::{Write, Result};

pub fn write_bmp(filename: &str, buffer: &[u8], width: u32, height: u32) -> Result<()> {
    let mut file = File::create(filename)?;
    let file_size = 54 + buffer.len() as u32;
    let header = vec![
        0x42, 0x4D,
        (file_size & 0xff) as u8, ((file_size >> 8) & 0xff) as u8, ((file_size >> 16) & 0xff) as u8, ((file_size >> 24) & 0xff) as u8,
        0, 0, 0, 0,
        54, 0, 0, 0,
        40, 0, 0, 0,
        (width & 0xff) as u8, ((width >> 8) & 0xff) as u8, ((width >> 16) & 0xff) as u8, ((width >> 24) & 0xff) as u8,
        (height & 0xff) as u8, ((height >> 8) & 0xff) as u8, ((height >> 16) & 0xff) as u8, ((height >> 24) & 0xff) as u8,
        1, 0,
        24, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    file.write_all(&header)?;
    file.write_all(buffer)?;
    Ok(())
}
