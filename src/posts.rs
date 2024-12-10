use crate::metadata::MetaData;

#[derive(Debug, Clone)]
pub struct Post {
    pub metadata: MetaData,
    pub content: String,
    pub path: String,
    pub public_link: String,
}

pub fn create_recent_posts_html(posts: &Vec<Post>, num_posts: usize) -> String {
    let mut recent_posts_html =
        String::from("<div id=\"recent-posts\">\n<h2>Recent Posts</h2>\n<ul>");

    let mut sorted_posts = posts.clone();
    sorted_posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));
    for post in sorted_posts.iter().take(num_posts) {
        recent_posts_html.push_str(&format!(
            "<li><a href=\"{}\">{} - [{}]</a></li>\n",
            post.path, post.metadata.title, post.metadata.date
        ));
    }

    recent_posts_html.push_str("<a href=\"./all.html\">» all posts</a>");
    recent_posts_html.push_str("</ul>\n</div>\n");
    recent_posts_html
}
