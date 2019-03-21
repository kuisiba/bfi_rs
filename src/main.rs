use std::env;
use std::fs::File;
use std::process;

//const MEM_SIZE: usize = 32768;

fn main() {
    let filename = parse(env::args()).unwrap_or_else(|err| {
        println!("parse err: {}", err);
        process::exit(1);
    });

    let ops = File::open(filename).unwrap_or_else(|err| {
        println!("can't open the file: {}", err);
        process::exit(1);
    });
    println!("{:?}", ops);
}
fn parse(args: env::Args) -> Result<String, String> {
    let args: Vec<String> = args.collect();
    if args.len() == 1 {
        return Err("not enough arguments".to_string());
    } else if args.len() == 2 {
        return Ok(args[1].clone());
    } else {
        return Err("invalid arguments".to_string());
    }
}
