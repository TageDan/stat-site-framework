// used for parsing metadata
use serde::de::DeserializeOwned;
use yaml_front_matter::YamlFrontMatter;

// used for parsing markdown
use pulldown_cmark::{Options, Parser};

use std::{
    fs::{self, read_to_string},
    path::Path,
};

pub fn parse_markdown(md_file: &Path) -> (String, String) {
    let md = read_to_string(md_file).expect(&format!("Can't find file {:?}", md_file));
    let meta_data_options = Options::ENABLE_YAML_STYLE_METADATA_BLOCKS;
    let parser = Parser::new_ext(&md, meta_data_options);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    (html, md)
}

pub fn get_metadata<T>(md: String) -> T
where
    T: DeserializeOwned,
{
    let result = YamlFrontMatter::parse::<T>(&md).unwrap();
    result.metadata
}

pub fn get_all_metadata<T: DeserializeOwned>(md_folder: &'static str) -> Vec<T> {
    let mut res = Vec::new();
    for file in fs::read_dir(md_folder).unwrap() {
        let file = file.unwrap().path();
        let content = fs::read_to_string(file).unwrap();
        let meta = get_metadata::<T>(content);
        res.push(meta)
    }
    res
}
