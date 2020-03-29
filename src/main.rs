use std::{io, env, fs};
use std::io::Read;
use crate::preprocessor::Preprocessor;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

mod preprocessor;

pub fn read_string_from_stdin() -> String {
    let mut response = String::new();
    io::stdin().read_to_string(&mut response).expect("Unable to read from stdin");
    return response;
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("Assemble")
            .short("a")
            .help("Assemble Hack ASM into machine code"))
        .arg(Arg::with_name("Preprocess")
            .short("p")
            .help("Preprocess Hack ASM code"))
        .arg(Arg::with_name("FILE")
            .help("Sets the input ASM file to use")
            .required(true)
            .index(1))
        .get_matches();


    let mut file = match fs::read_to_string(matches.value_of("FILE").unwrap()) {
        Ok(f) => f,
        Err(e) => panic!("Could not read file: {:?}", e),
    };

    if matches.is_present("Preprocess") {
        file = Preprocessor::new().process(file);
    }

    println!("{}", file);
}
