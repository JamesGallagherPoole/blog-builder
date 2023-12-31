use std::{fs, io::Error, path::Path};

pub fn read_file(path: &Path) -> Result<String, Error> {
    fs::read_to_string(path)
}

pub fn copy_dir_to(src_dir: &Path, dest_dir: &Path) -> Result<(), Error> {
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

pub fn write_to_file(directory: &Path, file_name: &str, contents: &str) -> Result<(), Error> {
    let path = Path::new(directory).join(file_name);
    fs::write(path, contents)?;
    Ok(())
}

pub fn create_html_file_name(file_name: &str) -> Option<String> {
    let path = Path::new(file_name);
    let file_stem = path.file_stem()?;
    let new_file_name = file_stem.to_string_lossy().into_owned() + ".html";
    Some(new_file_name)
}

pub fn prepend_go_up_folder_to_path(path: &str, num: i8) -> String {
    let mut new_path = String::new();
    for _ in 0..num {
        new_path.push_str("../");
    }
    new_path.push_str(path);
    new_path
}
