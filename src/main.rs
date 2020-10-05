mod trits;
mod colors;

use colors::*;

use std::{fs::File, io};
use std::io::prelude::*;
use std::io::Read;

const HELLO: &str = "Malbolge Interpeter by GWM";
const VERSION: &str = "1.0";

fn main() {
    if let Some(mut read) = process_args() {
        println!("{}...", "Initilizing memory".green());
        let mut mem: [trits::Tryte; trits::MAX as usize] = match init_mem(&mut read) {
            Ok(result) => result,
            Err(msg) => {
                println!("{}", msg);
                return;
            }
        };
        println!("{}!", "Memory initialized".green());
        let mut a: trits::Tryte = 0;
        let mut c: trits::Tryte = 0;
        let mut d: trits::Tryte = 0;
        println!("{}!", "Ready".green());
        let mut counter: u32 = 0;
        let mut input = io::stdin().bytes().into_iter();
        loop {
            counter += 1;
            let instruction = trits::add(c,  mem[c as usize]) % 94;
            match instruction {
                4 => {
                    c = mem[d as usize];
                },
                5 => {
                    io::stdout().write(&[(a % 256) as u8]).unwrap();
                },
                23 => {
                    a = match input.next() {
                        Some(res) => res.unwrap() as trits::Tryte,
                        None => trits::EOF
                    };
                },
                39 => {
                    let value = mem[d as usize];
                    let result = trits::move_right(value);
                    mem[d as usize] = result;
                    a = result;
                },
                40 => {
                    d = mem[d as usize];
                },
                62 => {
                    let result = trits::crazy(mem[d as usize], a);
                    mem[d as usize] = result;
                    a = result;
                },
                81 => {
                    break;
                },
                _ => {
                    //NOP (either 63 or invalid instruction)
                }
            }
            mem[c as usize] = trits::encrypt(mem[c as usize]);
            c = trits::add(c, 1);
            d = trits::add(d, 1);
        }
        println!("\n{} {} {}!", "Program has been finished in".green(), counter.to_string().blue(), "cycles".blue());
    } else {
        println!("{}!", "Program has been terminated".red())
    }
}

fn process_args() -> Option<Box<dyn Read>> {
    let args = std::env::args().collect::<Vec<String>>();
    let prog_name = &args[0];
    let len = args.len();
    if len == 1 {
        println!("{}...", "Using stdin as a source".green());
        Some(Box::new(io::stdin()))
    } else if len == 2 {
        let arg = &args[1];
        if arg.eq("--help") || arg.eq("-h") {
            println!("{} v. {}\nUsage: \n{} [--help | <file>]", HELLO, VERSION, prog_name);
            None
        } else {
            let file_name = arg;
            println!("{} '{}' {}...", "Using file".green(), file_name.blue(), "as a source".green());
            let file = match File::open(file_name) {
                Ok(file) => {
                    println!("{}...", "File has been opened successfully".green());
                    file
                },
                Err(err) => {
                    println!("{}: '{}'!", "Failed to open the file with error".red(), err.to_string().blue());
                    return None;
                }
            };
            Some(Box::new(file))
        }
    } else {
        println!("{}!", "Too many arguments".red());
        println!("Usage: \n{} [--help | [file]]", prog_name);
        None
    }
}

fn init_mem<T: Read>(src: &mut T) -> Result<[trits::Tryte; trits::MAX as usize], String> {
    let mut buf: Vec<u8> = Vec::new();
    println!("{}...", "Reading instructions from the source".green());
    if let Err(err) = src.read_to_end(&mut buf) {
        return Err(format!("{}: '{}'", "Failed to read source".red(), err.to_string().blue()));
    }
    println!("{}...", "Instructions have been read successfully".green());
    let mut mem: [trits::Tryte; trits::MAX as usize] = [0; trits::MAX as usize];
    let mut idx: usize = 0;
    for u in buf {
        let c: char = u.into();
        if !c.is_whitespace() {
            if idx >= trits::MAX as usize {
                return Err(format!("{} {} {}!", "There are more than".red(), trits::MAX.to_string().blue(), "instructions in the source".red()));
            }
            mem[idx] = u as trits::Tryte;
            idx += 1;
        }
    }
    if idx < 2 {
        return Err(format!("{} {} {}!", "There are less than".red(), 2, "instructions in the source".red()));
    }
    println!("{} {}...", idx.to_string().blue(), "Instructions have been loaded into memory successfully".green());
    for idx in idx..trits::MAX as usize {
        mem[idx] = trits::crazy(mem[idx-2], mem[idx-1]);
    }
    Ok(mem)
}