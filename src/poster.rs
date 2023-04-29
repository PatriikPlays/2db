use serde::{Deserialize, Serialize};
use std::io::{BufReader, ErrorKind, Read};
use std::path::PathBuf;
use std::fs::File;
use std::{fmt, io, u32};
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
    let data = read_file_as_string(file)?;

    let image: Img2d = serde_json::from_str(&data)?;

    return Ok(image);
}

pub fn read_2dja(file: &PathBuf) -> Result<Img2dArray,io::Error> {
    let data = read_file_as_string(file)?;

    let image_array: Img2dArray = serde_json::from_str(&data)?;

    return Ok(image_array);
}


pub fn read_2db(file: &PathBuf) -> Result<Img2d,io::Error> {
    let bytes = read_file_as_bytes(file)?;
    let bytes_length = bytes.len();

    let mut image = Img2d {
        label: None,
        tooltip: None,
        palette: vec![],
        pixels: vec![],
        width: 0,
        height: 0,
    };

    let mut ptr: usize = 0;

    //
    // Label
    //
    let label_length: u16 = {
        if ptr + 1 + 2 <= bytes_length {
            u16::from_le_bytes(bytes[ptr..ptr + 2].try_into().expect("Failed to get label length u16 (Input probably corrupted)"))
        } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get label length u16 (Buffer would overflow)")) }
    };
    ptr += 2;

    let mut label = String::new();
    if label_length != 0 {
        let start_ptr = ptr.clone();
        if start_ptr + 1 + label_length as usize <= bytes_length {
            for i in start_ptr..start_ptr+label_length as usize {
                let char = bytes[ptr] as char;
                if !char.is_ascii_control() {
                    label.push(char);
                } else {
                    println!("WARNING: Ignoring ASCII control character in label")
                }
                ptr += 1;
            }
        } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get label (Buffer would overflow)")) }
    }
    // Label END

    //
    // Tooltip
    //
    let tooltip_length: u16 = {
        if ptr + 1 + 2 <= bytes_length {
            u16::from_le_bytes(bytes[ptr..ptr + 2].try_into().expect("Failed to get tooltip length u16 (Input probably corrupted)"))
        } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get tooltip length u16 (Buffer would overflow)")) }
    };
    ptr += 2;

    let mut tooltip = String::new();
    if tooltip_length != 0 {
        let start_ptr = ptr.clone();
        if start_ptr + 1 + tooltip_length as usize <= bytes_length {
            for i in start_ptr..start_ptr+tooltip_length as usize {
                let char = bytes[ptr] as char;
                if !char.is_ascii_control() {
                    tooltip.push(char);
                } else {
                    println!("WARNING: Ignoring ASCII control character in tooltip")
                }
                ptr += 1;
            }
        } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get tooltip (Buffer would overflow)")) }
    }
    // Tooltip END

    //
    // Width and Height
    //
    let width: u32 = {
        if ptr + 1 + 4 <= bytes_length {
            u32::from_le_bytes(bytes[ptr..ptr + 4].try_into().expect("Failed to get width u32 (Input probably corrupted)"))
        } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get width u32 (Buffer would overflow)")) }
    };
    ptr += 4;

    let height: u32 = {
        if ptr + 1 + 4 <= bytes_length {
            u32::from_le_bytes(bytes[ptr..ptr + 4].try_into().expect("Failed to get height u32 (Input probably corrupted)"))
        } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get height u32 (Buffer would overflow)")) }
    };
    ptr += 4;
    // Width and Height END

    //
    // Palette
    //
    let palette_length: u8 = {
        if ptr + 1 + 1 <= bytes_length {
            u8::from_le_bytes(bytes[ptr..ptr + 1].try_into().expect("Failed to get palette length u8 (Input probably corrupted)"))
        } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get palette length u8 (Buffer would overflow)")) }
    };
    ptr += 1;

    let mut palette: Vec<u32> = Vec::new();
    if ptr + 1 + (palette_length as usize * 4) <= bytes_length {
        for i in (ptr..ptr+(palette_length as usize*4)).step_by(4) {
            palette.push(u32::from_le_bytes(bytes[i..i+4].try_into().expect("Failed to get palette (Input probably corrupted)")));
            ptr += 4;
        }
    } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get palette (Buffer would overflow)")) }
    // Palette END

    //
    // Pixels
    //
    let pixels_length: u32 = {
        if ptr + 1 + 4 <= bytes_length {
            u32::from_le_bytes(bytes[ptr..ptr + 4].try_into().expect("Failed to get pixels length u32 (Input probably corrupted)"))
        } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get pixels length u32 (Buffer would overflow)")) }
    };
    ptr += 4;

    let mut pixels: Vec<u8> = Vec::new();
    if ptr + 1 + pixels_length as usize <= bytes_length {
        for i in ptr..ptr+(pixels_length as usize) {
            pixels.push(bytes[i]);
        }
    } else { return Err(io::Error::new(ErrorKind::InvalidInput, "Could not get pixels (Buffer would overflow)")) }
    // Pixels END

    image.label = Some(label);
    image.tooltip = Some(tooltip);
    image.width = width;
    image.height = height;
    image.palette = palette;
    image.pixels = pixels;

    return Ok(image);
}

pub fn read_2dba(file: &PathBuf) -> Result<Img2dArray,io::Error> {
    let bytes = read_file_as_bytes(file)?;

    let mut imageArray = Img2dArray {
        width: 0,
        height: 0,
        title: None,
        pages: Vec::new(),
    };

    return Ok(imageArray);
}


