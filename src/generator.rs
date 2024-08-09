use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::render::Render;

pub struct MarkdownSiteGenerator {
    content: PathBuf,
    templates: PathBuf,
    output: PathBuf,
}

impl Default for MarkdownSiteGenerator {
    fn default() -> Self {
        Self {
            content: Path::new("./content").to_owned(),
            templates: Path::new("./templates").to_owned(),
            output: Path::new("./public").to_owned(),
        }
    }
}

impl MarkdownSiteGenerator {
    pub fn with_dirs<S>(content: Option<&S>, templates: Option<&S>, output: Option<&S>) -> Self
    where
        S: AsRef<OsStr>,
    {
        let (content, templates, output) = (
            {
                if let Some(x) = content {
                    Path::new(x).to_owned()
                } else {
                    Self::default().content
                }
            },
            {
                if let Some(x) = templates {
                    Path::new(x).to_owned()
                } else {
                    Self::default().templates
                }
            },
            {
                if let Some(x) = output {
                    Path::new(x).to_owned()
                } else {
                    Self::default().output
                }
            },
        );

        Self {
            content,
            templates,
            output,
        }
    }

    pub fn add_file_path(self, file_name: &str, render_tree: impl Render) -> Self {
        let html = render_tree.render(&self.templates);
        let file = file_name.to_owned() + ".html";
        let path = self.output.clone().join(file);
        fs::write(path, html).unwrap();
        self
    }

    pub fn add_content_folder_path<T: Serialize + DeserializeOwned>(
        self,
        folder_name: &str,
        content_render_tree: impl Render,
    ) -> Self {
        let output_folder = self.output.clone().join(folder_name);
        fs::create_dir_all(&output_folder).unwrap();
        for file in fs::read_dir(self.content.clone().join(folder_name)).unwrap() {
            let f = file.unwrap();
            let file_path = f.path();
            let file_stem = file_path.file_stem().unwrap();
            let file_name = file_stem.to_str().unwrap();
            let file_name = file_name;
            let html = content_render_tree.render_for_file::<T>(
                &self.templates,
                &self.content,
                &(folder_name.to_owned() + file_name),
            );
            let file = file_name.to_owned() + ".html";
            let path = output_folder.join(file);
            fs::write(path, html).unwrap();
        }
        self
    }
}
