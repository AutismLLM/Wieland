use serde::Deserialize;

use crate::rule::Language;

const CATEGORIES_YAML: &str = include_str!("../../../spec/taxonomy/categories.yaml");
const CATEGORIES_TYPESCRIPT_YAML: &str =
    include_str!("../../../spec/taxonomy/categories-typescript.yaml");
const CATEGORIES_GO_YAML: &str = include_str!("../../../spec/taxonomy/categories-go.yaml");

#[derive(Debug, Clone, Deserialize)]
pub struct Category {
    pub key: String,
    pub prefix: String,
}

#[derive(Debug, Deserialize)]
struct CategoriesFile {
    categories: Vec<Category>,
}

pub fn categories() -> Vec<Category> {
    categories_for(Language::Rust)
}

pub fn categories_for(language: Language) -> Vec<Category> {
    let yaml = match language {
        Language::Rust => CATEGORIES_YAML,
        Language::Typescript => CATEGORIES_TYPESCRIPT_YAML,
        Language::Go => CATEGORIES_GO_YAML,
    };
    #[allow(clippy::expect_used)]
    let parsed: CategoriesFile =
        serde_yaml_ng::from_str(yaml).expect("embedded taxonomy yaml is valid");
    parsed.categories
}

pub fn taxonomy_file(language: Language) -> &'static str {
    match language {
        Language::Rust => "taxonomy/categories.yaml",
        Language::Typescript => "taxonomy/categories-typescript.yaml",
        Language::Go => "taxonomy/categories-go.yaml",
    }
}

pub fn prefix_for(key: &str) -> Option<String> {
    categories()
        .into_iter()
        .find(|c| c.key == key)
        .map(|c| c.prefix)
}
