use std::{io::Error, path::Path};

use yaml_rust::YamlLoader;

use crate::files::read_file;

pub struct SiteConfig {
    pub title: String,
    pub url: String,
    pub description: String,
}

impl SiteConfig {
    pub fn read_site_config(input_dir: &Path) -> Result<SiteConfig, Error> {
        let config_path = Path::new(input_dir).join("config.yaml");
        match read_file(&config_path) {
            Ok(file_contents) => {
                let yaml = YamlLoader::load_from_str(&file_contents).unwrap();
                let yaml = &yaml[0];
                let title = yaml["title"].as_str().unwrap();
                let url = yaml["url"].as_str().unwrap();
                let description = yaml["description"].as_str().unwrap();
                Ok(SiteConfig {
                    title: title.to_string(),
                    url: url.to_string(),
                    description: description.to_string(),
                })
            }
            Err(e) => {
                println!("Error finding config file: {}", e);
                Err(e)
            }
        }
    }
}
