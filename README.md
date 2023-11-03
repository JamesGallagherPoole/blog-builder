## Blog Builder 🖳

This is a simple static site generator for my blog. It's written in Rust.
The tool is accessible for everyone to use but it's been designed quite specifically for my needs and workflow.
Do get in touch if you happen to try it out!

### Project Goals 🥅
- To be able write content for my blog in markdown.
- To have my site generated as I would like, based on those markdown files.
- Support Categories.
- Support RSS.
- HTML and CSS only. No Javascript or other faff.

### Why? 🤔
- To create a system for building my website from the bottom up that I understand.
- To create something that perfectly fits my needs.
- For the fun of it.

### Pre-requisites
- Install [Rust](https://www.rust-lang.org/tools/install).

### Setup
- Clone the repo.
- Run `cargo run -- --input example_template --output example_site` to generate the example site.

### Usage 📖
- Run `cargo run -- --input <input> --output <output>` to generate your site.
- The `input` folder should follow the format of the `example_template` folder.
  - You can customise the contents of the templates how you would like!
- The `output` folder will be created if it doesn't exist.

### Future Plans
- Auto compress and dither images.
- Show a preview of each post whenever they are shown, instead of just the title.

