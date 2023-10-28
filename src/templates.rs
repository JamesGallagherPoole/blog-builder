use std::{io::Error, path::Path};

use chrono::Datelike;

use crate::{
    files::{prepend_go_up_folder_to_path, read_file},
    posts::{create_recent_posts_html, Post},
};

/// get_header
///
/// Takes the path to the input directory and the number of levels down from that directory that we are generating for.
/// The levels down param controls how many "../" we need to prepend to the links within the header.
pub fn get_header(input_path: &Path, levels_down: i8) -> Result<String, Error> {
    let header_path = Path::new(input_path).join("header.md");
    let path_prepend_text = "../".repeat(levels_down as usize);

    match read_file(&header_path) {
        Ok(file_contents) => {
            let header_html = markdown::to_html(&file_contents);
            if levels_down > 0 {
                let wrapped_header = format!(
                    "<header>\n{}\n</header>\n",
                    header_html.replace("./", &path_prepend_text)
                );
                return Ok(wrapped_header);
            }
            let wrapped_header = format!("<header>\n{}\n</header>\n", header_html);
            Ok(wrapped_header)
        }
        Err(e) => {
            println!("Error finding header file: {}", e);
            Err(e)
        }
    }
}

/// get_footer
///
/// Takes the path to the input directory and the number of levels down from that directory that we are generating for.
/// The levels down param controls how many "../" we need to prepend to the links within the footer.
pub fn get_footer(input_path: &Path, levels_down: i8) -> Result<String, Error> {
    let footer_path = Path::new(input_path).join("footer.md");
    let path_prepend_text = "../".repeat(levels_down as usize);
    match read_file(&footer_path) {
        Ok(file_contents) => {
            let footer_html = markdown::to_html(&file_contents);
            if levels_down > 0 {
                let wrapped_footer = format!(
                    "<footer>\n{}\n</footer>\n",
                    footer_html.replace("./", &path_prepend_text)
                );
                return Ok(wrapped_footer);
            }
            let wrapped_footer = format!("<footer>\n{}\n</footer>\n", footer_html);
            Ok(wrapped_footer)
        }
        Err(e) => {
            println!("Error finding footer file: {}", e);
            Err(e)
        }
    }
}

pub fn get_index_template(input_path: &Path) -> Result<String, Error> {
    let index_path = input_path.to_str().unwrap().to_string() + "/index.html";
    match read_file(Path::new(index_path.as_str())) {
        Ok(file_contents) => Ok(file_contents),
        Err(e) => {
            println!("Error finding index file: {}", e);
            Err(e)
        }
    }
}

pub fn add_recent_posts(index_template: &str, posts: &Vec<Post>, num_posts: usize) -> String {
    let recent_posts_html = create_recent_posts_html(&posts, num_posts);

    let index_template = format!("{}\n{}", index_template, recent_posts_html);
    index_template
}

pub fn add_title_to_body(body: &str, title: &str) -> String {
    let body_with_title = format!("<h1>{}</h1>\n{}", title, body);
    body_with_title
}

pub fn wrap_in_header_and_footer(
    input_path: &Path,
    content_block: &str,
    levels_down: i8,
) -> Result<String, Error> {
    let header_block = get_header(input_path, levels_down)?;
    let footer_block = get_footer(input_path, levels_down)?;

    let wrapped_content = format!("{}{}{}", header_block, content_block, footer_block);
    let wrapped_in_container = format!(
        "\n<body>\n<div id=\"container\">{}</div></body>",
        wrapped_content
    );
    Ok(wrapped_in_container)
}

pub fn add_head(content_block: &str, title: &str, look_up: bool) -> Result<String, Error> {
    let mut style_path = "style/style.css".to_string();
    if look_up {
        style_path = prepend_go_up_folder_to_path(&style_path, 1)
    }

    let html_with_head = format!(
        "\n<head>\n<title>{}</title>\n<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n<link rel=\"stylesheet\" href=\"{}\">\n</head>\n{}",
        title,
        style_path,
        content_block
    );
    Ok(html_with_head)
}

pub fn group_by_year_as_html(posts: &Vec<Post>) -> String {
    let mut sorted_posts: Vec<(i32, Vec<Post>)> = Vec::new();
    for post in posts {
        let year = post.metadata.date.year();
        let mut found = false;
        for (i, sorted_post) in sorted_posts.iter_mut().enumerate() {
            if sorted_post.0 == year {
                sorted_post.1.push(post.clone());
                found = true;
                break;
            }
        }
        if !found {
            sorted_posts.push((year, vec![post.clone()]));
        }
    }
    // Sort by most recent year first
    sorted_posts.sort_by(|a, b| b.0.cmp(&a.0));

    let mut all_posts_html = String::new();
    for (year, posts) in sorted_posts {
        let mut year_html = format!("<h2>{}</h2>\n<ul>\n", year);
        // Sort by most recent post first
        let mut posts = posts;
        posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));

        for post in posts {
            year_html.push_str(
                format!(
                    "<li><a href=\"{}\">{}</a></li>\n",
                    post.path, post.metadata.title
                )
                .as_str(),
            );
        }
        all_posts_html.push_str(&year_html);
        all_posts_html.push_str("</ul>\n");
    }
    all_posts_html
}
