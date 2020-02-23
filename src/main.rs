use sha1::{Digest, Sha1};
use std::borrow::Cow;
use std::{env, fs, io};
use walkdir::WalkDir;

/// Get the filename of an entry from a directory walk, with backslashes replaced with forward slashes
fn unixy_filename_of(entry: &walkdir::DirEntry) -> String {
    return entry.path().display().to_string().replace("\\", "/");
}

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
            match fs::File::open(entry.path()) {
                Ok(mut file) => {
                    let mut hasher = Sha1::new();
                    io::copy(&mut file, &mut hasher).expect("io copying should work");
                    let result = hasher.result();
                    println!(
                        "{hash:x}  {path}",
                        hash = result,
                        path = unixy_filename_of(&entry)
                    );
                }
                Err(err) => {
                    let path = unixy_filename_of(&entry);

                    let tip = if path.len() > 260 {
                        // path is longer than normal windows limit - long path support might need to be enabled
                        Some(Cow::from(format!("NB: path length of {len} is greater than 260 limit; you might need to enable the 'Enable Win32 long paths' group policy setting", len = path.len())))
                    } else {
                        None
                    };
                    println!(
                        "Error:{err:?}{tip}  {path}",
                        err = err,
                        tip = tip.unwrap_or(Cow::from("")),
                        path = &path
                    )
                }
            }
        }
    }
}
