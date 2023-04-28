mod poster;

use clap::{arg, ArgMatches, command, value_parser};
use std::path::PathBuf;

fn main() {
    let matches = make_matches();

    let input = matches.get_one::<PathBuf>("input").expect("Input argument doesn't exist, this shouldn't have happened");
    let output = matches.get_one::<PathBuf>("output").expect("Output argument doesn't exist, this shouldn't have happened");
    let output_format = matches.get_one::<String>("outformat").expect("Output format doesn't exist, this shouldn't have happened");

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
            println!("Invalid output format supplied, valid formats are (json,binary)");
            return;
        }
    };

    if output_format_type == poster::ImgFormat::JSON {
        println!("jaeva scribe");
    } else if output_format_type == poster::ImgFormat::Binary {
        println!("bibnary");
    }

    //println!("{}",poster::read_2dj(input).unwrap());
}

fn make_matches() -> ArgMatches {
    return command!()
        .arg(
            arg!(-i --input <INPUT_FILE> "Sets input image file (use file extension to specify format)")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-o --output <OUTPUT_FILE> "Sets output file (extension is set automatically)")
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