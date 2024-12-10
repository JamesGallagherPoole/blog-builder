mod argparse;
mod category;
mod config;
mod files;
mod metadata;
mod paths;
mod posts;
mod rss;
mod templates;

use std::{
    fs::{self},
    io::Error,
    path::Path,
};

use argparse::Cli;
use category::{create_category_list_html, Category};
use clap::Parser;
use files::{create_html_file_name, write_to_file};
use paths::Paths;
use templates::{add_date_to_body, group_by_year_as_html};

use crate::{
    category::get_category_path,
    config::SiteConfig,
    files::{copy_dir_to, read_file},
    metadata::MetaData,
    posts::Post,
    rss::build_rss_feed,
    templates::{
        add_head, add_recent_posts, add_title_to_body, get_index_template,
        wrap_in_header_and_footer,
    },
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
                let config = SiteConfig::read_site_config(input_path)?;

                build_images_folder(input_path, output_path)?;
                build_style_folder(input_path, output_path)?;

                let (posts, categories) =
                    build_content_folder(&input_path, "posts", output_path, &config)?;

                build_main_page(input_path, output_path, &posts, &config.title)?;

                build_all_posts_page(input_path, output_path, &posts, &config.title)?;

                build_categories_index_page(input_path, output_path, &categories, &config.title)?;

                build_category_pages(input_path, output_path, &categories, &config.title)?;

                build_rss_feed(output_path, posts, &config);

                println!("Done!");
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}

fn read_args() -> Result<Paths, Error> {
    let args = Cli::parse();

    let paths = Paths {
        input: args.input_dir.to_str().unwrap().to_string(),
        output: args.output_dir.to_str().unwrap().to_string(),
    };
    Ok(paths)
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
    config: &SiteConfig,
) -> Result<(Vec<Post>, Vec<(Category, Vec<Post>)>), std::io::Error> {
    let mut posts: Vec<Post> = Vec::new();
    let mut categories: Vec<(Category, Vec<Post>)> = Vec::new();

    let path_to_build = Path::new(input_dir).join(folder_to_build);
    if path_to_build.is_dir() {
        for entry in fs::read_dir(&path_to_build)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(file_contents) = read_file(&path).ok() {
                let (file_metadata, file_contents) =
                    MetaData::read_metadata_and_contents(&file_contents);

                let mut post_html = markdown::to_html(&file_contents);
                post_html = add_date_to_body(&post_html, &file_metadata.date);
                let html_body = add_title_to_body(&post_html, &file_metadata.title);
                let wrapped_html = wrap_in_header_and_footer(&input_dir, &html_body, 0)?;
                let wrapped_html_with_head = add_head(&wrapped_html, &file_metadata.title, false)?;
                let html_file_name = create_html_file_name(&path.to_str().unwrap()).unwrap();
                fs::create_dir_all(&output_dir)?;
                println!("Writing {} to {}", html_file_name, &output_dir.display());
                write_to_file(&output_dir, &html_file_name, &wrapped_html_with_head)?;

                let link_path = format!("./{}", html_file_name);

                let public_link =
                    format!("{}/{}{}", config.url, output_dir.display(), html_file_name);

                let post = Post {
                    metadata: file_metadata.clone(),
                    content: wrapped_html_with_head,
                    path: link_path,
                    public_link: public_link,
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

fn build_main_page(
    input_dir: &Path,
    output_dir: &Path,
    posts: &Vec<Post>,
    site_title: &str,
) -> Result<(), Error> {
    if input_dir.is_dir() {
        // find a path within this directory called index.md
        for entry in fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.to_string_lossy().contains("index.html") {
                if path.is_file() {
                    println!("Found main template. Building and copying to destination...");
                    let index_template = get_index_template(input_dir)?;
                    let index_content = add_recent_posts(&index_template, &posts, 10);
                    let wrapped_index = wrap_in_header_and_footer(&input_dir, &index_content, 0)?;
                    let wrapped_index_with_head = add_head(&wrapped_index, site_title, false)?;
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
    site_title: &str,
) -> Result<(), Error> {
    if input_dir.is_dir() {
        let content = group_by_year_as_html(&posts);
        let wrapped_index = wrap_in_header_and_footer(&input_dir, &content, 0)?;
        let wrapped_index_with_head = add_head(&wrapped_index, site_title, false)?;
        write_to_file(output_dir, "all.html", &wrapped_index_with_head)?;
    }
    Ok(())
}

fn build_categories_index_page(
    input_dir: &Path,
    output_dir: &Path,
    categories: &Vec<(Category, Vec<Post>)>,
    site_title: &str,
) -> Result<(), Error> {
    if input_dir.is_dir() {
        let mut content = String::from("<h2>Categories</h2>\n<ul>\n");

        for (category, posts) in categories {
            let category_list = create_category_list_html(category, posts);
            content.push_str(&category_list);
        }
        let wrapped_index = wrap_in_header_and_footer(&input_dir, &content, 0)?;
        let wrapped_index_with_head = add_head(&wrapped_index, site_title, false)?;
        write_to_file(output_dir, "categories.html", &wrapped_index_with_head)?;
    }
    Ok(())
}

fn build_category_pages(
    input_dir: &Path,
    output_dir: &Path,
    categories: &Vec<(Category, Vec<Post>)>,
    site_title: &str,
) -> Result<(), Error> {
    if input_dir.is_dir() {
        for (category, posts) in categories {
            let content = &create_category_list_html(category, posts);
            let wrapped_index = wrap_in_header_and_footer(&input_dir, &content, 0)?;
            let wrapped_index_with_head = add_head(&wrapped_index, site_title, false)?;
            write_to_file(output_dir, &category.path, &wrapped_index_with_head)?;
        }
    }
    Ok(())
}
