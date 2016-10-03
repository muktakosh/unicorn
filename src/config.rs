//! Configuration parser and generator module

use std::io::{Read, Error, ErrorKind};
use std::fs::File;

pub fn load(f: &str) -> Result<String, Error> {
    let c = try!(load_file(f));
    // TODO: Parse the config file
    Ok(c)
}

fn load_file(f: &str) -> Result<String, Error> {
    let mut file = try!(File::open(f));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    Ok(contents)
}
