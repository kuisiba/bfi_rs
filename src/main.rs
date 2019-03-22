use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

const MEM_SIZE: usize = 32768;

fn read_file(f: File) -> Vec<char> {
    let mut ret = Vec::new();
    for byte in f.bytes() {
        if let Ok(c) = byte {
            // + -  . , > < [ ]
            if c == 43 || c == 45 || c == 46 || c == 44 || c == 62 || c == 60 || c == 91 || c == 93
            {
                ret.push(c as char);
            }
        }
    }
    if MEM_SIZE < ret.len() {
        eprintln!("too long codes");
        process::exit(1);
    }
    ret
}
fn parse_args(args: env::Args) -> Result<String, String> {
    let args: Vec<String> = args.collect();
    if args.len() == 1 {
        Err("not enough arguments".to_string())
    } else if args.len() == 2 {
        Ok(args[1].clone())
    } else {
        Err("invalid arguments".to_string())
    }
}
fn getchar() -> char {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap_or_else(|err| {
        eprintln!(", read_line err: {}", err);
        process::exit(1);
    });
    let c: char = s.trim().parse::<char>().unwrap_or_else(|err| {
        eprintln!("parse char err: {}", err);
        process::exit(1);
    });
    c
}
fn run(codes: Vec<char>) {
    let codes_len = codes.len(); //bfソースコードの長さ
    let mut pc = 0; //実行する命令の位置
    let mut memory = vec![0; MEM_SIZE];
    let mut ptr = 0; //メモリのポインタ用
    let mut stack = VecDeque::new(); //ループ用 [ の位置をPUSH ] が来たらPOP
    while pc < codes_len {
        if codes[pc] == '>' {
            ptr += 1;
            if MEM_SIZE <= ptr {
                eprintln!("memory[] out of bounds. ptr: {}", ptr);
                process::exit(1);
            }
        } else if codes[pc] == '<' {
            ptr -= 1;
        } else if codes[pc] == '+' {
            memory[ptr] += 1;
            if 127 < memory[ptr] {
                eprintln!("memory[ptr]: {}", memory[ptr]);
                process::exit(1);
            }
        } else if codes[pc] == '-' {
            memory[ptr] -= 1;
            if 127 < memory[ptr] {
                eprintln!("memory[ptr]: {}", memory[ptr]);
                process::exit(1);
            }
        } else if codes[pc] == '.' {
            print!("{}", memory[ptr] as char);
        } else if codes[pc] == ',' {
            let c = getchar() as u8;
            memory[ptr] = c;
        } else if codes[pc] == '[' {
            stack.push_back(pc);
            //メモリの値が0なら対応するカッコまでジャンプ
            if memory[ptr] == 0 {
                let mut bracket = 1;
                while 0 < bracket {
                    pc += 1;
                    if codes_len <= pc {
                        //対応す括弧が見つからなかった
                        eprintln!("] not found");
                        process::exit(1);
                    }
                    if codes[pc] == '[' {
                        bracket += 1;
                    } else if codes[pc] == ']' {
                        bracket -= 1;
                    }
                }
                stack.pop_back().unwrap();
            }
        } else if codes[pc] == ']' {
            pc = stack.pop_back().unwrap() - 1;
        }
        pc += 1;
    }
    println!();
    io::stdout().flush().unwrap();
}
fn main() {
    let filename = parse_args(env::args()).unwrap_or_else(|err| {
        eprintln!("args parse err: {}", err);
        process::exit(1);
    });

    let f = File::open(filename).unwrap_or_else(|err| {
        eprintln!("can't open the file: {}", err);
        process::exit(1);
    });
    let codes: Vec<char> = read_file(f);
    run(codes);
}
