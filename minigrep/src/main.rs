use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let conf = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", conf.query);
    println!("In file {}", conf.filename);
    if let Err(e) = run(conf) {
        println!("Application error : {}", e);
        process::exit(1);
    }
}
