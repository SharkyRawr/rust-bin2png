extern crate image;

use image::{Rgb};
use std::env;
use std::path::Path;
use std::io::{Read, BufReader, Error};


fn print_help() {
    println!("rust-bin2png {} - by Sophie 'Sharky' Schumann", env!("CARGO_PKG_VERSION"));
    println!("Requires one argument:\tinfile - File to convert to PNG");
}

fn main() -> Result<(),std::io::Error> {
    println!("Hello, world!");

    if env::args().len() <= 1 {
        print_help();
        std::process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    let infilepath = Path::new(&args[1]);
    let outfilepath = format!("{}.png", infilepath.file_stem().unwrap().to_str().unwrap());

    println!("Trying to read '{}' ...", infilepath.display());

    let infile = std::fs::File::open(infilepath)?;
    #[allow(non_snake_case)]
    let mut infileReader = std::io::BufReader::new(&infile);

    let filelen = infile.metadata().unwrap().len();
    println!("Filelen is {} bytes ...", filelen);

    
    let root = (filelen as f64 / 3.0f64).sqrt();
    let width = root.floor() as u32; // truncate me daddy
    let height = width;
    if width != root as u32 {
        println!("!! Warning: Resulting image not perfectly square, data will be truncated!\n");
    }
    println!("Image size is: {}x{}", width, height);

    let mut img = image::RgbImage::new(width, height);
    
    let mut buf: [u8; 3] = [0; 3];

    for y in 0..height {
        for x in 0..width {
            infileReader.read(&mut buf).expect("read failed");
            img.put_pixel(x, y, Rgb(buf));
        }
    }

    img.save(outfilepath).expect("Image saved!");
    Ok(())
}
