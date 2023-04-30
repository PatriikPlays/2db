mod poster;

use clap::{arg, command, value_parser, ArgMatches};
use std::fs;
use std::path::PathBuf;

fn main() {
    let matches = make_matches();

    let input = matches
        .get_one::<PathBuf>("input")
        .expect("Input argument doesn't exist, this shouldn't have happened");
    let output = matches
        .get_one::<PathBuf>("output")
        .expect("Output argument doesn't exist, this shouldn't have happened");
    let output_format = matches
        .get_one::<String>("outformat")
        .expect("Output format doesn't exist, this shouldn't have happened");

    if !input.exists() {
        println!("Input file doesn't exist.");
        return;
    }
    if input.is_dir() {
        println!("Input can't be a directory.");
        return;
    }

    if output.is_dir() {
        println!("Output can't be a directory.");
        return;
    }

    if !output.parent().unwrap().exists() {
        println!("Output file parent directory doesn't exist.");
        return;
    }

    let output_format_type: poster::ImgFormat = match output_format.to_lowercase().as_str() {
        "json" => poster::ImgFormat::JSON,
        "j" => poster::ImgFormat::JSON,
        "binary" => poster::ImgFormat::Binary,
        "bin" => poster::ImgFormat::Binary,
        "b" => poster::ImgFormat::Binary,
        _ => {
            println!("Invalid output format supplied, valid formats are (json,binary).");
            return;
        }
    };

    let input_file_extension = match input.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => {
            println!("Input file has no extension, has to be 2dj/2dja/2db/2dba.");
            return;
        }
    };

    let (input_format_type, is_image_array) = match input
        .extension()
        .expect("Failed to get extension of input file.")
        .to_ascii_lowercase()
        .to_str()
        .expect("Failed to convert input file extension to str.")
    {
        "2dj" => (poster::ImgFormat::JSON, false),
        "2db" => (poster::ImgFormat::Binary, false),
        "2dja" => (poster::ImgFormat::JSON, true),
        "2dba" => (poster::ImgFormat::Binary, true),
        _ => {
            println!("Invalid input file file extension, has to be 2dj/2dja/2db/2dba.");
            return;
        }
    };

    if is_image_array {
        let image_array: poster::Img2dArray;

        if input_format_type == poster::ImgFormat::JSON {
            image_array = match poster::read_2dja(input) {
                Ok(t) => t,
                Err(e) => {
                    println!("Failed to read input image array (2dja): {}", e);
                    return;
                }
            };
        } else if input_format_type == poster::ImgFormat::Binary {
            image_array = match poster::read_2dba(input) {
                Ok(t) => t,
                Err(e) => {
                    println!("Failed to read input image array (2dba): {}", e);
                    return;
                }
            };
        } else {
            panic!("Invalid input format type");
        }

        let mut out_path = output.clone();
        if output_format_type == poster::ImgFormat::JSON {
            out_path.set_extension("2dja");
            fs::write(
                out_path,
                serde_json::to_string(&image_array).expect("Failed to parse image data to JSON"),
            )
            .expect("Failed to write to output file.");
        } else if output_format_type == poster::ImgFormat::Binary {
            out_path.set_extension("2dba");
            fs::write(
                out_path,
                poster::img_2d_array_to_bytes(&image_array)
                    .expect("Failed to parse image data to bytes"),
            )
            .expect("Failed to write to output file.");
        }
    } else {
        let image: poster::Img2d;

        if input_format_type == poster::ImgFormat::JSON {
            image = match poster::read_2dj(input) {
                Ok(t) => t,
                Err(e) => {
                    println!("Failed to read input image (2dj): {}", e);
                    return;
                }
            };
        } else if input_format_type == poster::ImgFormat::Binary {
            image = match poster::read_2db(input) {
                Ok(t) => t,
                Err(e) => {
                    println!("Failed to read input image (2db): {}", e);
                    return;
                }
            };
        } else {
            panic!("Invalid input format type");
        }

        let mut out_path = output.clone();
        if output_format_type == poster::ImgFormat::JSON {
            out_path.set_extension("2dj");
            fs::write(
                out_path,
                serde_json::to_string(&image).expect("Failed to parse image data to JSON"),
            )
            .expect("Failed to write to output file.");
        } else if output_format_type == poster::ImgFormat::Binary {
            out_path.set_extension("2db");
            fs::write(
                out_path,
                poster::img_2d_to_bytes(&image).expect("Failed to parse image data to bytes"),
            )
            .expect("Failed to write to output file.");
        }
    }
}

fn make_matches() -> ArgMatches {
    return command!()
        .arg(
            arg!(-i --input <INPUT_FILE> "Sets input image file (use file extension to specify format)")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-o --output <OUTPUT_FILE> "Sets output file (extension is set automatically, do not set one)")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-F --outformat <FORMAT> "Output format (\"binary\" or \"json\")")
                .required(true)
                .value_parser(value_parser!(String))
        )
        .get_matches();
}
