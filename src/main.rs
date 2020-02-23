use sha1::{Sha1, Digest};
use walkdir::WalkDir;
use std::{fs, io, env};


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: expecting exactly 1 argument");
        eprintln!("Usage: {} DIRECTORY", args[0]);
        eprintln!("\n  DIRECTORY   Root directory to start sha1 summing from");
    }
    let root = args.get(1).expect("first argument must be root path");

    for entry in WalkDir::new(root).same_file_system(true) {
        let entry = entry.expect("able to read file");
        if entry.file_type().is_file() {
            let mut file = fs::File::open(entry.path()).expect("should be able to open file");
            let mut hasher = Sha1::new();
            io::copy(&mut file, &mut hasher).expect("io copying should work");
            let result = hasher.result();
            println!("{hash:x}  {file}", hash=result, file=entry.path().display().to_string().replace("\\", "/"));
        }
    }
}
