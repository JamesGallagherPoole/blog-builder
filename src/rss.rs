extern crate rss;

use rss::{ChannelBuilder, Guid, ItemBuilder};
use std::path::Path;

use crate::config::SiteConfig;
use crate::files::write_to_file;
use crate::posts::Post;

pub fn build_rss_feed(output_dir: &Path, posts: Vec<Post>, config: &SiteConfig) {
    let mut items = Vec::new();
    for post in posts {
        let item = ItemBuilder::default()
            .title(Some(post.metadata.title.clone()))
            .link(Some(post.public_link.clone()))
            .description(Some(post.metadata.summary.clone()))
            .pub_date(Some(post.metadata.rss_formatted_date()))
            .guid(Some(Guid {
                value: post.public_link.clone(),
                ..Default::default()
            }))
            .build()
            .unwrap();
        items.push(item);
    }

    let channel = ChannelBuilder::default()
        .title(&config.title)
        .link(&config.url)
        .description(&config.description)
        .items(items)
        .build()
        .unwrap();

    let output = channel.to_string();

    write_to_file(output_dir, "feed.xml", &output).unwrap();
}
