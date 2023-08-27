mod category;
mod files;
mod metadata;
mod paths;
mod posts;
mod templates;

use std::{
    env,
    fs::{self},
    io::Error,
    path::Path,
};

use category::{create_category_list_html, Category};
use files::{create_html_file_name, write_to_file};
use paths::Paths;

use crate::{
    category::get_category_path,
    files::{copy_dir_to, read_file, remove_until_first_slash},
    metadata::MetaData,
    posts::Post,
    templates::{add_head, add_recent_posts, get_index_template, wrap_in_header_and_footer},
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

                let (posts, categories) = build_content_folder(&input_path, "posts", output_path)?;

                build_main_page(input_path, output_path, &posts)?;

                build_all_posts_page(input_path, output_path, &posts)?;

                build_categories_index_page(input_path, output_path, &categories)?;

                build_category_pages(input_path, output_path, &categories)?;
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

    if args.len() > 3 || args.len() <= 1 {
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

fn build_content_folder(
    input_dir: &Path,
    folder_to_build: &str,
    output_dir: &Path,
) -> Result<(Vec<Post>, Vec<(Category, Vec<Post>)>), std::io::Error> {
    let mut posts: Vec<Post> = Vec::new();
    let mut categories: Vec<(Category, Vec<Post>)> = Vec::new();

    let path_to_build = Path::new(input_dir).join(folder_to_build);
    let output_dir = Path::new(output_dir).join(&folder_to_build);
    if path_to_build.is_dir() {
        for entry in fs::read_dir(&path_to_build)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(file_contents) = read_file(&path).ok() {
                let (file_metadata, file_contents) =
                    MetaData::read_metadata_and_contents(&file_contents);

                let html = markdown::to_html(&file_contents);
                let wrapped_html = wrap_in_header_and_footer(&input_dir, &html, 1)?;
                let wrapped_html_with_head = add_head(&wrapped_html, true)?;
                let html_file_name = create_html_file_name(&path.to_str().unwrap()).unwrap();
                fs::create_dir_all(&output_dir)?;
                println!("Writing {} to {}", html_file_name, &output_dir.display());
                write_to_file(&output_dir, &html_file_name, &wrapped_html_with_head)?;

                let link_path = format!(
                    "./{}/{}",
                    remove_until_first_slash(&output_dir.display().to_string()),
                    html_file_name
                );

                let post = Post {
                    metadata: file_metadata.clone(),
                    content: wrapped_html_with_head,
                    path: link_path,
                };

                posts.push(post.clone());

                // Process Categories
                for category in &file_metadata.categories {
                    let category_path = get_category_path(&category);
                    let category = Category {
                        name: category.clone(),
                        path: category_path.clone(),
                    };

                    let mut found = false;
                    for (cat, posts) in &mut categories {
                        if cat.name == category.name {
                            found = true;
                            posts.push(post.clone());
                        }
                    }
                    if !found {
                        categories.push((category, vec![post.clone()]));
                    }
                }
            }
        }
    }

    Ok((posts, categories))
}

fn build_main_page(input_dir: &Path, output_dir: &Path, posts: &Vec<Post>) -> Result<(), Error> {
    if input_dir.is_dir() {
        // find a path within this directory called index.md
        for entry in fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.to_string_lossy().contains("index.html") {
                if path.is_file() {
                    println!("Found main template. Building and copying to destination...");
                    let index_template = get_index_template(input_dir)?;
                    let index_content = add_recent_posts(&index_template, &posts, 5);
                    let wrapped_index = wrap_in_header_and_footer(&input_dir, &index_content, 0)?;
                    let wrapped_index_with_head = add_head(&wrapped_index, false)?;
                    write_to_file(output_dir, "index.html", &wrapped_index_with_head)?;
                }
            }
        }
    }
    Ok(())
}

fn build_all_posts_page(
    input_dir: &Path,
    output_dir: &Path,
    posts: &Vec<Post>,
) -> Result<(), Error> {
    if input_dir.is_dir() {
        let index_content = add_recent_posts("", &posts, 5);
        let wrapped_index = wrap_in_header_and_footer(&input_dir, &index_content, 0)?;
        let wrapped_index_with_head = add_head(&wrapped_index, false)?;
        write_to_file(output_dir, "all.html", &wrapped_index_with_head)?;
    }
    Ok(())
}

fn build_categories_index_page(
    input_dir: &Path,
    output_dir: &Path,
    categories: &Vec<(Category, Vec<Post>)>,
) -> Result<(), Error> {
    if input_dir.is_dir() {
        let mut content = String::from("<h2>Categories</h2>\n<ul>\n");

        for (category, posts) in categories {
            let category_list = create_category_list_html(category, posts);
            content.push_str(&category_list);
        }
        let wrapped_index = wrap_in_header_and_footer(&input_dir, &content, 0)?;
        let wrapped_index_with_head = add_head(&wrapped_index, false)?;
        write_to_file(output_dir, "categories.html", &wrapped_index_with_head)?;
    }
    Ok(())
}

fn build_category_pages(
    input_dir: &Path,
    output_dir: &Path,
    categories: &Vec<(Category, Vec<Post>)>,
) -> Result<(), Error> {
    if input_dir.is_dir() {
        for (category, posts) in categories {
            let content = &create_category_list_html(category, posts);
            let wrapped_index = wrap_in_header_and_footer(&input_dir, &content, 0)?;
            let wrapped_index_with_head = add_head(&wrapped_index, false)?;
            write_to_file(output_dir, &category.path, &wrapped_index_with_head)?;
        }
    }
    Ok(())
}
