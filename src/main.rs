use std::fs::File;
use std::io::{BufRead,BufReader,Lines,Result};
use std::path::Path;


fn main() {
    println!("Hello, world!");
    read_deps();
}





fn read_deps() {
    lines("input.txt");
}

fn lines<P>(file_name: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(file_name)?;
        Ok(BufReader::new(file).lines())
}