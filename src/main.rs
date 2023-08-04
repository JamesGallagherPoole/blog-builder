mod files;
mod paths;
mod templates;

use std::{
    env,
    fs::{self},
    io::Error,
    path::Path,
};

use files::{create_html_file_name, write_to_file};
use paths::Paths;

use crate::{
    files::{copy_dir_to, read_file},
    templates::{get_footer, get_header},
};

fn main() -> Result<(), Error> {
    match read_args() {
        Ok(path) => {
            println!("Input Path: {}", path.input);
            println!("Output Path: {}", path.output);

            let input_path = Path::new(&path.input);
            let output_path = Path::new(&path.output);

            create_directory(path.output.as_str()).expect("Unable to create directory");

            if input_path.is_dir() {
                build_images_folder(input_path, output_path)?;
                build_style_folder(input_path, output_path)?;

                let header_block = get_header(&path.input)?;
                let footer_block = get_footer(&path.input)?;

                build_markdown_folder(
                    &input_path,
                    "posts",
                    output_path,
                    &header_block,
                    &footer_block,
                )?;
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
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

fn build_markdown_folder(
    input_dir: &Path,
    folder_to_build: &str,
    output_dir: &Path,
    header: &str,
    footer: &str,
) -> Result<(), std::io::Error> {
    let path_to_build = Path::new(input_dir).join(folder_to_build);
    let output_dir = Path::new(output_dir).join(folder_to_build);
    if path_to_build.is_dir() {
        for entry in fs::read_dir(path_to_build)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let new_input_path = Path::new(input_dir).join(folder_to_build);
                build_markdown_folder(
                    &new_input_path,
                    entry.file_name().to_str().unwrap(),
                    &output_dir,
                    header,
                    footer,
                )?;
            } else {
                if let Some(file_contents) = read_file(&path).ok() {
                    let html = markdown::to_html(&file_contents);
                    let html_file_name = create_html_file_name(&path.to_str().unwrap()).unwrap();
                    fs::create_dir_all(&output_dir)?;
                    write_to_file(&output_dir, &html_file_name, &html)?;
                }
            }
        }
    }
    Ok(())
}
