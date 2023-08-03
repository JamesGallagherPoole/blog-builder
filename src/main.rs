mod paths;

use std::{
    env,
    fs::{self, DirEntry},
    io::Error,
    path::Path,
};

use paths::Paths;

fn main() {
    match read_args() {
        Ok(path) => {
            println!("Input Path: {}", path.input);
            println!("Output Path: {}", path.output);

            if let Some(content) = read_file(&path.input).ok() {
                println!("Content: {}", markdown::to_html(&content));
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn read_args() -> Result<Paths, Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 3 {
        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Provide a single path argument",
        ))
    } else {
        let paths = Paths {
            input: args[1].clone(),
            output: args[2].clone(),
        };
        Ok(paths)
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
                if let Some(ext) = path.extension() {}
            }
        }
    }
    Ok(())
}

fn read_file(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}
