use std::env;
use std::fs;
use std::path::Path;

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
            println!("Error: expected 0 or 1 argument, but {} were provided.\nConsider putting argument in quotation marks.\n", args.len());
            print_usage();
            exit();
        },
    };
}

fn list_dir(path: &Path){
    // TODO: check for * in the input, and search files accordingly
    // but for now: if the path is a file, convert it to the parent directory
    let path = match path.is_file() {
        true => path.parent().unwrap(),
        false => path,
    };

    let entries = match fs::read_dir(path) {
        Ok(x) => x,
        Err(_x) => {
            println!("error: directory not found");
            exit(); // exits, panic is to avoid needing a type `std::fs::ReadDir`
            panic!();
        },
    };

    let entries = entries.collect::<Vec<_>>();
    let length = entries.len();
    let mut files: Vec<String> = Vec::with_capacity(length);
    let mut folders: Vec<String> = Vec::with_capacity(length);

    for entry in entries {
        if let Ok(entry) = entry {
            let name_string = match entry.file_name().into_string() {
                    Ok(x) => x,
                    Err(_x) => panic!("error getting file name")
                };

            if entry.path().is_dir() { // folder
                // Here, `entry` is a `DirEntry`.
                folders.push(name_string);
            } else { // file
                // Here, `entry` is a `DirEntry`.
                files.push(name_string);
            }
        }
    }

    // yes, I could only print if something is in folders or files,
    //    but this makes it more obvious when nothing is there.
    let mut print_string = String::from("---Folders---\n");
    for folder in folders {
        print_string += &folder;
        print_string += "\n";
    }
    print_string += "\n---Files---\n";
    for file in files{
        print_string += &file;
        print_string += "\n";
    }
    print!("{}", print_string);

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
    println!("ls");
    println!("ls <path/to/folder/>");
    println!("ls <path/to/file.ext>\n");
    println!("Outputs the files and folders of current folder './' if no arguments given.");
    println!("Outputs the files and folders in a given folder");
    println!("Outputs the files and folders in the same folder as a given file\n");
}

fn exit() {
    std::process::exit(0);
}