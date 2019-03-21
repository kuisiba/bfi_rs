use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

const MEM_SIZE: usize = 32768;

fn main() {
    let filename = parse_args(env::args()).unwrap_or_else(|err| {
        println!("args parse err: {}", err);
        process::exit(1);
    });

    let f = File::open(filename).unwrap_or_else(|err| {
        println!("can't open the file: {}", err);
        process::exit(1);
    });
    let ops: Vec<u8> = read_file(f);
}
fn read_file(f: File) -> Vec<u8> {
    let mut ret = Vec::new();
    for byte in f.bytes() {
        if let Ok(c) = byte {
            // + -  . , > < [ ]
            if c == 43 || c == 45 || c == 46 || c == 44 || c == 62 || c == 60 || c == 91 || c == 93
            {
                ret.push(c);
            }
        }
    }
    ret.truncate(MEM_SIZE);
    ret
}

fn parse_args(args: env::Args) -> Result<String, String> {
    let args: Vec<String> = args.collect();
    if args.len() == 1 {
        return Err("not enough arguments".to_string());
    } else if args.len() == 2 {
        return Ok(args[1].clone());
    } else {
        return Err("invalid arguments".to_string());
    }
}
