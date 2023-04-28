use serde::{Deserialize, Serialize};
use std::io::{BufReader, ErrorKind, Read};
use std::path::PathBuf;
use std::fs::File;
use std::{fmt, io};
use std::fmt::Formatter;

#[derive(Serialize, Deserialize)]
pub struct Img2d {
    pub label: Option<String>,
    pub tooltip: Option<String>,
    pub palette: Vec<u32>,
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32
}

impl fmt::Display for Img2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}",serde_json::to_string(self).unwrap())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Img2dArray {
    pub width: u32,
    pub height: u32,
    pub title: Option<String>,
    pub pages: Vec<Img2d>
}

impl fmt::Display for Img2dArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}",serde_json::to_string(self).unwrap())
    }
}

#[derive(PartialEq)]
pub enum ImgFormat {
    Binary,
    JSON
}


fn read_file_as_bytes(file: &PathBuf) -> Result<Vec<u8>, io::Error> {
    let f = File::open(&file)?;

    let mut reader = BufReader::new(f);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;

    return Ok(buffer);
}

fn read_file_as_string(file: &PathBuf) -> Result<String, io::Error> {
    let f = File::open(&file)?;

    let mut reader = BufReader::new(f);
    let mut data: String = String::new();
    reader.read_to_string(&mut data)?;

    return Ok(data);
}

pub fn read_2dj(file: &PathBuf) -> Result<Img2d,io::Error> {
    let data = read_file_as_string(&file)?;

    let image: Img2d = serde_json::from_str(&data)?;

    return Ok(image);
}

pub fn read_2dja(file: &PathBuf) -> Result<Img2dArray,io::Error> {
    let data = read_file_as_string(&file)?;

    let image_array: Img2dArray = serde_json::from_str(&data)?;

    return Ok(image_array);
}

/*
pub fn read_2db(file: PathBuf) -> Result<Img2d,io::Error> {
    let bytes = read_file_as_bytes(file)?;
    let bytes_length = bytes.len();

    let mut image = Img2d {
        label: "".to_string(),
        tooltip: "".to_string(),
        palette: vec![],
        pixels: vec![],
        width: 0,
        height: 0,
    };

    let mut ptr: usize = 0;

    let label_length: u16= {
        if ptr + 1 + 2 <= bytes_length {
            ptr += 2;
            u16::from_le_bytes(bytes[ptr..ptr + 2].try_into().expect("Failed to get label length u16 (Input probably corrupted)"))
        } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get label length u16 (Buffer would overflow)")) }
    };
    let label: String = {
        let mut str = String::new();
        if label_length == 0 { str }
        if ptr + 1 + label_length as usize <= bytes_length {
            for i in ptr..ptr+label_length as usize {
                str.push(u8::from_le(0) as char);
            }
        } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get label (Buffer would overflow)")) }
        str
    };

    /*
    let label_length = u16::from_le_bytes(bytes[ptr..ptr+2].try_into().unwrap());
    ptr += 2;
    image.label = String::from_utf8_lossy(&bytes[ptr..ptr+label_length as usize]).to_string();
    ptr += label_length as usize;

    let tooltip_length = u16::from_le_bytes(bytes[ptr..ptr+2].try_into().unwrap());
    ptr += 2;
    image.tooltip = String::from_utf8_lossy(&bytes[ptr..ptr+tooltip_length as usize]).to_string();
    ptr += tooltip_length as usize;

    image.width = u32::from_le_bytes(bytes[ptr..ptr+4].try_into().unwrap());
    ptr += 4;

    image.height = u32::from_le_bytes(bytes[ptr..ptr+4].try_into().unwrap());
    ptr += 4;

    let palette_length = u32::from_le_bytes(bytes[ptr..ptr+4].try_into().unwrap());
    ptr += 4;
    for i in 0..palette_length {
        image.palette.push(u32::from_le_bytes(bytes[ptr+i as usize*4..ptr+i as usize*4+4].try_into().unwrap()));
    }
    ptr += palette_length as usize * 4;

    let pixels_length = u32::from_le_bytes(bytes[ptr..ptr+4].try_into().unwrap());
    ptr += 4;
    for i in 0..pixels_length {
        image.pixels.push(bytes[ptr+i as usize] );
    }
     */

    return Ok(image);
}

pub fn read_2dba(file: PathBuf) -> Result<Img2dArray,io::Error> {
    let bytes = read_file_as_bytes(file)?;

    let mut imageArray = Img2dArray {
        width: 0,
        height: 0,
        title: "".to_string(),
        pages: Vec::new(),
    };

    return Ok(imageArray);
}
*/

