use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::process;

const MEM_SIZE: usize = 32768;

fn getchar() -> Result<char, Box<Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let c: char = s.trim().parse::<char>()?;
    if c.is_ascii() {
        Ok(c)
    } else {
        Err("it is not ascii".into())
    }
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
fn read_file(s: String) -> Vec<char> {
    let mut ret = Vec::new();
    for c in s.chars() {
        if c == '+'
            || c == '-'
            || c == '.'
            || c == ','
            || c == '>'
            || c == '<'
            || c == '['
            || c == ']'
        {
            ret.push(c);
        }
    }
    if MEM_SIZE < ret.len() {
        eprintln!("too long codes");
        process::exit(1);
    }
    ret
}
fn run(codes: Vec<char>) -> Result<(), Box<Error>> {
    let codes_len = codes.len(); //bfソースコードの長さ
    let mut pc = 0; //実行する命令の位置
    let mut memory = vec![0; MEM_SIZE];
    let mut ptr = 0; //メモリのポインタ用
    let mut stack = VecDeque::new(); //ループ用 [ の位置をPUSH ] が来たらPOP
    while pc < codes_len {
        if codes[pc] == '>' {
            ptr += 1;
            if MEM_SIZE <= ptr {
                return Err(format!("memory[] out of bounds. ptr: {}", ptr).into());
            }
        } else if codes[pc] == '<' {
            ptr -= 1;
        } else if codes[pc] == '+' {
            memory[ptr] += 1;
            if 127 < memory[ptr] {
                return Err(format!("memory[{}]: {}", ptr, memory[ptr]).into());
            }
        } else if codes[pc] == '-' {
            memory[ptr] -= 1;
            if 127 < memory[ptr] {
                return Err(format!("memory[{}]: {}", ptr, memory[ptr]).into());
            }
        } else if codes[pc] == '.' {
            print!("{}", memory[ptr] as char);
        } else if codes[pc] == ',' {
            match getchar() {
                Ok(c) => {
                    memory[ptr] = c as u8;
                }
                Err(e) => {
                    eprintln!("getchar() failed: {}", e);
                    process::exit(1);
                }
            }
        } else if codes[pc] == '[' {
            stack.push_back(pc);
            //メモリの値が0なら対応するカッコまでジャンプ
            if memory[ptr] == 0 {
                let mut bracket = 1;
                while 0 < bracket {
                    pc += 1;
                    if codes_len <= pc {
                        //対応する括弧が見つからなかった
                        return Err(format!("] not found").into());
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
    Ok(())
}
fn main() {
    let filename = parse_args(env::args()).unwrap_or_else(|err| {
        eprintln!("args parse err: {}", err);
        process::exit(1);
    });

    let s = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("can't open the file: {}", err);
        process::exit(1);
    });
    let codes: Vec<char> = read_file(s);
    if let Err(e) = run(codes) {
        eprintln!("run error: {}", e);
        process::exit(1);
    };
}
