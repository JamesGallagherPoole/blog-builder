use crate::posts::Post;

#[derive(Debug)]
pub struct Category {
    pub name: String,
    pub path: String,
}

pub fn get_category_path(category: &str) -> String {
    let prefix = category
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase();

    format!("{}.html", prefix)
}

pub fn create_category_list_html(category: &Category, posts: &Vec<Post>) -> String {
    let mut category_list_html = String::from("<div id=\"category-list\">\n<h2>");
    category_list_html.push_str(&category.name);
    category_list_html.push_str("</h2>\n<ul>\n");

    // Sort posts so that the most recent is first
    let mut sorted_posts = posts.clone();
    sorted_posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));

    for post in sorted_posts {
        let post_html = format!(
            "<li><a href=\"{}\">{} - [{}]</a></li>\n",
            post.path,
            post.metadata.title,
            post.metadata.date.format("%-d %B %Y").to_string()
        );
        category_list_html.push_str(&post_html);
    }

    category_list_html.push_str("</ul>\n</div>\n");

    category_list_html
}
