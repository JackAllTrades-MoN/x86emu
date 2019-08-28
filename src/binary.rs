use std::fs::File;
use std::io::Read;

pub type BinFile = Vec<u8>;


pub fn load(filename: &str) -> BinFile {
    let mut file = File::open(filename).expect("file not found");
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)
        .expect("something went wrong reading the file");
    buf
}
