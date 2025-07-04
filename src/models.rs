use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisteredModelSearchResult {
    pub registered_models: Vec<RegisteredModel>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegisteredModel {
    pub name: String,
}
