extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args; //helps to take user inputs
use std::fs::File;
use std::io::copy; //copy the file contents to compressed file
use std::io::BufReader; //reads the contents before copying
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        //min number of args should be 3-> cargo run , source filename ,
        //compressed filename
        eprintln!("Usage: `source` `target`");
        return;
    }
    //take input as 2nd args of the user input
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    //provide output for 3rd args of the user input
    let output = File::create(args().nth(2).unwrap()).unwrap();
    //does all the compresing
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    //copy all the input file data to output data
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    //input file size
    println!("Source len {:?}", input.get_ref().metadata().unwrap().len());
    //output file size
    println!("Target len: {:?}", output.metadata().unwrap().len());
    //gives the time taken
    println!("Elapsed: {:?}", start.elapsed());
}
