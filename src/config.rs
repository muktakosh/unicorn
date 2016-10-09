//! Configuration parser and generator module

use std::io::{Read, Write, Error, ErrorKind};
use std::fs::File;
use std::path::Path;
use schema::config_schema::{Config, Service};
use serde_json;

pub fn load(f: &str) -> Result<Config, Error> {
    let c = try!(load_file(f));
    let conf: Config = match serde_json::from_str(&c[..]) {
        Ok(c) => c,
        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
    };
    Ok(conf)
}

/// Create a default config
pub fn default() -> Config {
    let mut conf = Config::new();
    conf.services.insert("api".to_string(),
                         Service::new("127.0.0.1".to_string(), 60000));
    conf.services.insert("datastore".to_string(),
                         Service::new("127.0.0.1".to_string(), 60001));
    conf
}

/// Write default config data to `unicorn.json` in the current dir.
pub fn init() -> Result<(), Error> {
    let confpath = Path::new("unicorn.json");
    if confpath.exists() {
        return Err(Error::new(ErrorKind::AlreadyExists, "unicorn.json already exists."));
    }

    let out = match serde_json::to_string_pretty(&default()) {
        Ok(o) => o,
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    };

    let mut f = try!(File::create(confpath));
    try!(f.write_all(out.as_bytes()));
    Ok(())
}

fn load_file(f: &str) -> Result<String, Error> {
    let mut file = try!(File::open(f));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    Ok(contents)
}
