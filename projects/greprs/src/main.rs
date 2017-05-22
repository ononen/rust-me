extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
                println!("problem parsing arguments: {}", err);
                process::exit(1);
            });

    println!("searching for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        println!("application error: {}", e) ;

        process::exit(1);
    }

}

