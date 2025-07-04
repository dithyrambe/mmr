use anyhow::Result;
use reqwest::Client;

use crate::models::{
    ModelVersion, ModelVersionSearchResult, RegisteredModel, RegisteredModelSearchResult,
};

pub struct MlflowClient {
    pub tracking_uri: String,
    pub client: Client,
}

impl MlflowClient {
    pub fn new(tracking_uri: String) -> Self {
        let client = Client::new();

        MlflowClient {
            tracking_uri,
            client,
        }
    }

    pub async fn list_models(&self, pattern: &str) -> Result<()> {
        let url = format!(
            "{}/api/2.0/mlflow/registered-models/search",
            self.tracking_uri
        );
        let sql_pattern = if pattern.contains('*') {
            pattern.replace("*", "%")
        } else {
            format!("%{pattern}%")
        };

        let mut page_token: Option<String> = None;
        let mut models: Vec<RegisteredModel> = Vec::new();

        loop {
            let mut query_params = vec![("filter", format!("name LIKE '{sql_pattern}'"))];

            if let Some(token) = &page_token {
                query_params.push(("page_token", token.clone()));
            }

            let response = self.client.get(&url).query(&query_params).send().await?;

            match response.json::<RegisteredModelSearchResult>().await {
                Ok(result) => {
                    models.extend(result.registered_models);
                    page_token = result.next_page_token;
                    if page_token.is_none() {
                        break;
                    }
                }
                Err(_) => {
                    println!("No model found with that pattern");
                    break;
                }
            }
        }
        for model in models {
            println!("{}", model.name);
        }

        Ok(())
    }

    pub async fn list_versions(&self, model_name: &str) -> Result<()> {
        let url = format!("{}/api/2.0/mlflow/model-versions/search", self.tracking_uri);

        let mut page_token: Option<String> = None;
        let mut versions: Vec<ModelVersion> = Vec::new();

        loop {
            let mut query_params = vec![("filter", format!("name='{model_name}'"))];

            if let Some(token) = &page_token {
                query_params.push(("page_token", token.clone()));
            }

            let response = self.client.get(&url).query(&query_params).send().await?;

            match response.json::<ModelVersionSearchResult>().await {
                Ok(result) => {
                    versions.extend(result.model_versions);
                    page_token = result.next_page_token;
                    if page_token.is_none() {
                        break;
                    }
                }
                Err(_) => {
                    println!("No model found with that pattern");
                    break;
                }
            }
        }
        for version in versions.iter().rev() {
            println!("{}/{}\t{}", version.name, version.version, version.source);
        }

        Ok(())
    }
}
