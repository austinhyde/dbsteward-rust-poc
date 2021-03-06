// allows us to use load_yaml!
#[macro_use]
extern crate clap;
// allows us to use #[derive(Deserialise)] and #[serde()]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use std::fs::File;
use std::io::prelude::*;

// declare that xml.rs is a standalone module with its own namespace
mod xml;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let left_file = matches.value_of("old").unwrap();
    let mut f = File::open(left_file).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("could not read file");

    let dbsteward: xml::DBSteward = serde_xml_rs::deserialize(contents.as_bytes()).unwrap();
    println!("{:?}", dbsteward)
}
