use std::collections::HashMap;
use std::io::BufReader;
use std::io;
use std::fs::File;
use std::io::prelude::*;

const DEFAULT_CONFIG_TEXT: &'static [u8] =
b"width=640
height=480
fullscreen=false";

const CONFIG_FILE_NAME: &'static str = "config.ini";

fn write_default_config() -> Result<(), io::Error> {
    let mut f = try!(File::create(CONFIG_FILE_NAME));
    f.write_all(DEFAULT_CONFIG_TEXT);
    Ok(())
}

pub fn load_config_file() -> HashMap<String, String> {
    let mut fr = File::open(CONFIG_FILE_NAME);

    let mut f;

    match fr {
        Ok(x) => f = x,
        Err(x)        => {
            println!("Failed to open config file. Writing and loading the default configuration.");

            match write_default_config() {
                Ok(x)  => return load_config_file(),
                Err(x) => { panic!("Failed to write configuration."); }
            }
        }
    }

    let mut reader = BufReader::new(&f);

    let mut result: HashMap<String, String> = HashMap::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split("=").collect();
        let name = tokens[0];
        let value = tokens[1];

        result.insert(name.to_string(), value.to_string());
    }

    result
}

