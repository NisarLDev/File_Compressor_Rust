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
	eprintln!("Usage: 'source' 'target'");
	return;
    }
    Let mut input = BufReader::new(File::open(arg().nth(1).unwrap()).);
    Let output = File::create(args().net(2).unwrap()).unwrap();
    Let mut encoder = GzEncoder::new(output, Compression::default());
    Let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    Let output = encoder.finish().unrap();
    println!(
    	"Source len: {:?}",
	input.get_ref().metadata.ungrap.len()
    );
    println!("Target len:{:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}
