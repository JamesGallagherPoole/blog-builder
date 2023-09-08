use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct MetaData {
    pub title: String,
    pub date: NaiveDate,
    pub categories: Vec<String>,
    pub summary: String,
}

impl MetaData {
    /// read_metadata_and_contents
    ///
    /// Takes a string slice of the file contents and returns a tuple of the metadata and the content.
    /// It reads the metadata block from the top of the markdown files and remove them afterwards
    pub fn read_metadata_and_contents(file_contents: &str) -> (MetaData, &str) {
        let (yaml, content) = frontmatter::parse_and_find_content(file_contents).unwrap();

        let mut metadata = MetaData {
            title: String::from(""),
            date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            categories: Vec::new(),
            summary: String::from(""),
        };

        if let Some(yaml_hash) = yaml {
            if let Some(yaml) = yaml_hash.as_hash() {
                if let Some(title) = yaml.get(&yaml_rust::Yaml::from_str("title")) {
                    metadata.title = title.as_str().unwrap().to_string();
                }
                if let Some(date) = yaml.get(&yaml_rust::Yaml::from_str("date")) {
                    metadata.date =
                        NaiveDate::parse_from_str(date.as_str().unwrap(), "%Y-%m-%d").unwrap();
                }
                if let Some(categories) = yaml.get(&yaml_rust::Yaml::from_str("categories")) {
                    if let Some(categories) = categories.as_vec() {
                        for category in categories {
                            metadata
                                .categories
                                .push(category.as_str().unwrap().to_string());
                        }
                    }
                }
            }
        }

        (metadata, content)
    }
}
