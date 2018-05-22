use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = parse(&args); // parse
    let filename = match filename {
        Some(x) => x,
        None => {
            print_usage();
            exit();
            String::from("")
        }
    };

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("error reading the file");
    
    println!("{}", contents);
}

fn parse(args: &[String]) -> Option<String> {
    let args: &[String] = &args[1..args.len()];
    for arg in args {
        if arg == "-h" || arg == "--help" {
            let filename: Option<String> = None;
            return filename;
        }
    }
    if args.len() != 1 {
        println!("Error: expected one argument, but received {}.\n", args.len());
        let filename: Option<String> = None;
        return filename;
    }
    let filename = Some(args[0].clone());
    return filename;
}

fn print_usage() {
    println!("USAGE:");
    println!("\ncat <path/to/filename.ext>\n");
    println!("Outputs the contents of filename.ext\n\n");
}

fn exit() {
    std::process::exit(0);
}