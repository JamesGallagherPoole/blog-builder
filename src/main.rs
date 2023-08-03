use std::{
    env,
    fs::{self, DirEntry},
    io::Error,
    path::Path,
};

fn main() {
    match read_args() {
        Ok(path) => {
            println!("Path: {}", path);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn read_args() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Provide a single path argument",
        ))
    } else {
        Ok(args[1].clone())
    }
}

// Function to visit files in a directory
fn visit_files(dir: &Path, cb: &dyn Fn(&DirEntry)) -> Result<(), Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_files(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
