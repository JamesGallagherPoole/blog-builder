use std::{io::Error, path::Path};

use crate::files::read_file;

pub fn get_header(input_path: &str) -> Result<String, Error> {
    let header_path = input_path.to_string() + "/header.md";
    match read_file(Path::new(header_path.as_str())) {
        Ok(file_contents) => {
            let header_html = markdown::to_html(&file_contents);
            let wrapped_header = format!("<header>\n{}\n</header>\n", header_html);
            Ok(wrapped_header)
        }
        Err(e) => {
            println!("Error finding header file: {}", e);
            Err(e)
        }
    }
}

pub fn get_footer(input_path: &str) -> Result<String, Error> {
    let footer_path = input_path.to_string() + "/footer.md";
    match read_file(Path::new(footer_path.as_str())) {
        Ok(file_contents) => {
            let footer_html = markdown::to_html(&file_contents);
            let wrapped_footer = format!("<footer>\n{}\n</footer>\n", footer_html);
            Ok(wrapped_footer)
        }
        Err(e) => {
            println!("Error finding footer file: {}", e);
            Err(e)
        }
    }
}

pub fn get_index(input_path: &Path) -> Result<String, Error> {
    let index_path = input_path.to_str().unwrap().to_string() + "/index.html";
    match read_file(Path::new(index_path.as_str())) {
        Ok(file_contents) => Ok(file_contents),
        Err(e) => {
            println!("Error finding index file: {}", e);
            Err(e)
        }
    }
}

pub fn wrap_in_header_and_footer(
    content_block: &str,
    header_block: &str,
    footer_block: &str,
) -> Result<String, Error> {
    let wrapped_content = format!("{}{}{}", header_block, content_block, footer_block);
    let wrapped_in_container = format!("\n<div id=\"container\">{}</div>", wrapped_content);
    Ok(wrapped_in_container)
}

pub fn add_head(content_block: &str) -> Result<String, Error> {
    let html_with_head = format!(
        "\n<head>\n<viewport content=\"width=device-width, initial-scale=1.0\">\n<link rel=\"stylesheet\" href=\"./style/style.css\">\n</head>\n{}",
        content_block
    );
    Ok(html_with_head)
}
