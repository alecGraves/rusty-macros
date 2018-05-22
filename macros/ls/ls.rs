use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsString;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    parse(&args);
    Ok(())
}

fn parse(args: &[String]){
    let args: &[String] = &args[1..args.len()]; // ignore .exe path
    check_help(args);
    match args.len() {
        0 => list_dir(&Path::new(".")),
        1 => list_dir(&Path::new(&args[0])),
        _ => {
            print_usage();
            exit();
        },
    };
}

fn list_dir(dir: &Path){
    if let Ok(entries) = fs::read_dir(dir) {
        let entries = entries.collect::<Vec<_>>();
        let length = entries.len();
        let mut files: Vec<OsString> = Vec::with_capacity(length);
        let mut folders: Vec<OsString> = Vec::with_capacity(length);
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_dir() { // folder
                    // Here, `entry` is a `DirEntry`.
                    folders.push(entry.file_name());
                } else { // file
                    // Here, `entry` is a `DirEntry`.
                    files.push(entry.file_name());
                }
            }
        }
        println!("---Folders---");
        for folder in folders{
            print!("{:?}\t", folder);
        }
        println!();
        println!();
        println!("---Files---");
        for file in files{
            print!("{:?}\t", file);
        }
        println!();
    }
}

fn check_help(args: &[String]){
    for arg in args {
        if arg == "-h" || arg == "--help" {
            print_usage();
            exit();
        }
    }
}

fn print_usage() {
    println!("USAGE:");
    println!("ls <path/to/path1/> ... <path/to/pathN/>\n");
    println!("Outputs the files and folders in path1 ... pathN");
    println!("Outputs the files and folders of current path if no arguments given.\n\n");
}

fn exit() {
    std::process::exit(0);
}