use crate::markdown_utils::{get_metadata, parse_markdown};
use std::{fs, path::Path};

use handlebars::{to_json, Handlebars};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value as Json};

pub trait Render {
    fn handlebars() -> Handlebars<'static> {
        Handlebars::new()
    }
    fn render(&self, template_folder: &Path) -> String;
    fn render_for_file<T>(
        &self,
        template_folder: &Path,
        content_folder: &Path,
        file_name: &str,
    ) -> String
    where
        T: Serialize,
        T: DeserializeOwned;
}

impl<C> Render for (&str, C)
where
    C: Render,
{
    fn render(&self, template_folder: &Path) -> String {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        Self::handlebars()
            .render_template(
                &template,
                &json!({"content": self.1.render(template_folder)}),
            )
            .unwrap()
    }
    fn render_for_file<T>(
        &self,
        template_folder: &Path,
        content_folder: &Path,
        file_name: &str,
    ) -> String
    where
        T: Serialize,
        T: DeserializeOwned,
    {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        Self::handlebars()
            .render_template(
                &template,
                &json!({"content": self.1.render_for_file::<T>(template_folder, content_folder, file_name)}),
            )
            .unwrap()
    }
}

impl Render for &str {
    fn render(&self, template_folder: &Path) -> String {
        let file = self.to_owned().to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        Self::handlebars()
            .render_template(&template, &json!({}))
            .unwrap()
    }
    fn render_for_file<T>(
        &self,
        template_folder: &Path,
        content_folder: &Path,
        file_name: &str,
    ) -> String
    where
        T: Serialize + DeserializeOwned,
    {
        let file = self.to_owned().to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        let file = file_name.to_owned() + ".md";
        let (html, md) = parse_markdown(&content_folder.join(file));
        let mut json = to_json(get_metadata::<T>(md));
        merge(&mut json, &json!({"content": html}));
        Self::handlebars()
            .render_template(&template, &json)
            .unwrap()
    }
}

impl Render for (&str, Json) {
    fn render_for_file<T>(
        &self,
        template_folder: &Path,
        content_folder: &Path,
        file_name: &str,
    ) -> String
    where
        T: Serialize,
        T: DeserializeOwned,
    {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        let file = file_name.to_owned() + ".md";
        let (html, md) = parse_markdown(&content_folder.join(file));
        let mut json = to_json(get_metadata::<T>(md));
        merge(&mut json, &self.1);
        merge(&mut json, &json!({"content": html}));
        Self::handlebars()
            .render_template(&template, &json)
            .unwrap()
    }
    fn render(&self, template_folder: &Path) -> String {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(&file)).expect(&format!(
            "Can't find file {:?}",
            template_folder.join(&file)
        ));
        Self::handlebars()
            .render_template(&template, &self.1)
            .unwrap()
    }
}

fn merge(a: &mut Json, b: &Json) {
    match (a, b) {
        (&mut Json::Object(ref mut a), &Json::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Json::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

impl<C> Render for (&str, Json, C)
where
    C: Render,
{
    fn render(&self, template_folder: &Path) -> String {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        let mut json = self.1.clone();
        merge(
            &mut json,
            &json!({"content": self.2.render(template_folder)}),
        );
        Self::handlebars()
            .render_template(&template, &json)
            .unwrap()
    }
    fn render_for_file<T>(
        &self,
        template_folder: &Path,
        content_folder: &Path,
        file_name: &str,
    ) -> String
    where
        T: Serialize,
        T: DeserializeOwned,
    {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        let mut json = self.1.clone();
        merge(
            &mut json,
            &json!({"content": self.2.render_for_file::<T>(template_folder, content_folder, file_name)}),
        );
        Self::handlebars()
            .render_template(&template, &json)
            .unwrap()
    }
}
