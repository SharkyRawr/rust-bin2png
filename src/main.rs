extern crate image;

use image::{Rgb};
use std::env;
use std::path::Path;
use std::io::Read;
use std::time::Instant;


fn print_help() {
    println!("rust-bin2png {} - by Sophie 'Sharky' Schumann", env!("CARGO_PKG_VERSION"));
    println!("Requires one argument:\tinfile - File to convert to PNG");
}

fn main() -> Result<(),Box<dyn std::error::Error>> {
    println!("Hello, world!");

    if env::args().len() <= 1 {
        print_help();
        std::process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    let infilepath = Path::new(&args[1]);
    let outfilepath = infilepath.with_extension("png");

    println!("Trying to read '{}' ...", infilepath.display());

    let infile = std::fs::File::open(infilepath)?;
    let mut infile_reader = std::io::BufReader::new(&infile);

    let filelen = infile.metadata().unwrap().len();
    println!("Filelen is {} bytes ...", filelen);

    
    let root = ((filelen / 3) as f64).sqrt();
    let width = root.floor() as u32; // truncate me daddy
    let height = width;
    if width as f64 != root {
        println!("!! Warning: Input data does not fit perfectly in a square, data will be truncated!");
    }
    println!("Image size is: {}x{}", width, height);

    let mut img = image::RgbImage::new(width, height);
    let mut buf: [u8; 3] = [0; 3];

    let start_instant = Instant::now();
    for y in 0..height {
        for x in 0..width {
            infile_reader.read(&mut buf).expect("read failed");
            img.put_pixel(x, y, Rgb(buf));
        }
    }

    println!("Writing image to '{}' ...", outfilepath.to_str().unwrap());
    img.save(outfilepath).expect("unable to save image?");
    println!("Done! Took {:?}.", start_instant.elapsed());
    Ok(())
}
