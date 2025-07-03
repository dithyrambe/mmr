use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisteredModelSearchResult {
    pub registered_models: Vec<RegisteredModel>,
}

#[derive(Debug, Deserialize)]
pub struct RegisteredModel {
    pub name: String,
}
