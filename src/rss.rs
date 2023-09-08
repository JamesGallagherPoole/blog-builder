extern crate rss;

use rss::{ChannelBuilder, Guid, ItemBuilder};
use std::path::Path;

use crate::files::write_to_file;
use crate::posts::Post;

pub fn build_rss_feed(output_dir: &Path, posts: Vec<Post>) {
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
        .title("James Poole".to_string())
        .link("https://james.poole.ie".to_string())
        .description("Notes and thoughts on game development and my process.".to_string())
        .items(items)
        .build()
        .unwrap();

    let output = channel.to_string();

    write_to_file(output_dir, "feed.xml", &output).unwrap();
}
