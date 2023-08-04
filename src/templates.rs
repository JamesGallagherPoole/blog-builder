use std::{io::Error, path::Path};

use crate::files::read_file;

pub fn get_header(input_path: &str) -> Result<String, Error> {
    let header_path = input_path.to_string() + "/header.md";
    match read_file(Path::new(header_path.as_str())) {
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

pub fn get_footer(input_path: &str) -> Result<String, Error> {
    let footer_path = input_path.to_string() + "/footer.md";
    match read_file(Path::new(footer_path.as_str())) {
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
