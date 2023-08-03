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

            if let Some(file_contents) = read_file(&path.input).ok() {
                let html = markdown::to_html(&file_contents);
                let html_file_name = create_new_file_name(&path.input).unwrap();
                write_to_file(&path.output, &html_file_name, &html).expect("Unable to write file");
            }

            create_directory(path.output.as_str()).expect("Unable to create directory");
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

fn create_directory(path: &str) -> Result<(), Error> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn write_to_file(directory: &str, file_name: &str, contents: &str) -> Result<(), Error> {
    let path = Path::new(directory).join(file_name);
    fs::write(path, contents)?;
    Ok(())
}

fn create_new_file_name(file_name: &str) -> Option<String> {
    let path = Path::new(file_name);
    let file_stem = path.file_stem()?;
    let new_file_name = file_stem.to_string_lossy().into_owned() + ".html";
    Some(new_file_name)
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
