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

            let input_path = Path::new(&path.input);
            let output_path = Path::new(&path.output);

            if input_path.is_dir() {
                build_images_folder(input_path, output_path);
                build_style_folder(input_path, output_path);
                /*
                let html = markdown::to_html(&file_contents);
                let html_file_name = create_new_file_name(&path.input).unwrap();
                write_to_file(&path.output, &html_file_name, &html).expect("Unable to write file");
                 */
                let header_block = get_header(&path.input);
                let footer_block = get_footer(&path.input);
            }
            if let Some(file_contents) = read_file(&path.input).ok() {}

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

fn get_header(input_path: &str) -> Result<String, Error> {
    let header_path = input_path.to_string() + "/header.md";
    match read_file(header_path.as_str()) {
        Ok(file_contents) => {
            let header_html = markdown::to_html(&file_contents);
            let wrapped_header = format!("<header>\n{}\n</header>", header_html);
            Ok(wrapped_header)
        }
        Err(e) => {
            println!("Error finding header file: {}", e);
            Err(e)
        }
    }
}

fn get_footer(input_path: &str) -> Result<String, Error> {
    let footer_path = input_path.to_string() + "/footer.md";
    match read_file(footer_path.as_str()) {
        Ok(file_contents) => {
            let footer_html = markdown::to_html(&file_contents);
            let wrapped_footer = format!("<footer>\n{}\n</footer>", footer_html);
            Ok(wrapped_footer)
        }
        Err(e) => {
            println!("Error finding footer file: {}", e);
            Err(e)
        }
    }
}

fn build_images_folder(input_dir: &Path, output_dir: &Path) -> Result<(), Error> {
    if input_dir.is_dir() {
        // find a path within this directory called images/
        for entry in fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.to_string_lossy().contains("images") {
                if path.is_dir() {
                    println!("Found images folder. Copying to destination...");
                    let output_images_path = output_dir.join("images");
                    copy_dir_to(&path, &output_images_path)?;
                }
            }
        }
    }
    Ok(())
}

fn build_style_folder(input_dir: &Path, output_dir: &Path) -> Result<(), Error> {
    if input_dir.is_dir() {
        // find a path within this directory called style/
        for entry in fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.to_string_lossy().contains("style") {
                if path.is_dir() {
                    println!("Found style folder. Copying to destination...");
                    let output_style_path = output_dir.join("style");
                    copy_dir_to(&path, &output_style_path)?;
                }
            }
        }
    }
    Ok(())
}

fn copy_dir_to(src_dir: &Path, dest_dir: &Path) -> Result<(), Error> {
    if !dest_dir.exists() {
        fs::create_dir_all(dest_dir)?;
    }
    for entry_result in fs::read_dir(src_dir)? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            // Copy the file
            fs::copy(entry.path(), dest_dir.join(entry.file_name()))?;
        } else if file_type.is_dir() {
            // Recursively copy the directory
            copy_dir_to(&entry.path(), &dest_dir.join(entry.file_name()))?;
        }
    }
    Ok(())
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
