extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;


fn main(){
    if args().len()!=3{
	eprintln("Usage: 'source' 'target'");
	return;
    }
    Let mut input = BufReader::new(File::open(arg().nth(1).unwrap()).)

}
