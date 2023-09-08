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
    for post in posts.iter().rev().take(num_posts) {
        let post_html = format!(
            "<li><a href=\"{}\">{}</a></li>\n",
            post.path, post.metadata.title
        );
        recent_posts_html.push_str(&post_html);
    }
    recent_posts_html.push_str("<a href=\"./all.html\">» all posts</a>");
    recent_posts_html.push_str("</ul>\n</div>\n");
    recent_posts_html
}
