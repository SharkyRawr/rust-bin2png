extern crate image;
use image::{Rgb};

use indicatif::{ProgressBar, ProgressStyle};

use std::env;
use std::path::Path;
use std::io::Read;
use std::time::Instant;


fn print_help() {
    println!("rust-bin2png {} - by Sophie 'Sharky' Schumann", env!("CARGO_PKG_VERSION"));
    println!("Requires one argument:\t  infile  - File to convert to PNG");
    println!("Optional second argument: outfile - Filename/path to save as, will always be PNG");
}

fn main() -> Result<(),Box<dyn std::error::Error>> {
    println!("Hello, world!");

    if env::args().len() <= 1 {
        print_help();
        std::process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    let infilepath = Path::new(&args[1]);
    let mut outfilepath: String = String::from(infilepath.with_extension("png").to_str().unwrap());

    if env::args().len() == 3 {
        outfilepath = String::from(Path::new(&args[2]).with_extension("png").to_str().unwrap());
    }

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
    let pb = ProgressBar::new(filelen);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {bytes:>7}/{total_bytes:7} @ {bytes_per_sec} eta {eta} {msg}")
        .progress_chars("##-"));

    for y in 0..height {
        for x in 0..width {
            infile_reader.read(&mut buf).expect("read failed");
            img.put_pixel(x, y, Rgb(buf));
        }
        pb.inc(width as u64*3);
    }
    pb.finish();

    println!("Writing image to '{}' ...", outfilepath);
    img.save(outfilepath).expect("unable to save image?");    
    println!("Done! Took {:?}.", start_instant.elapsed());
    Ok(())
}
