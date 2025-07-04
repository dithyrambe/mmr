use anyhow::Result;
use reqwest::Client;

use crate::{RegisteredModelSearchResult, models::RegisteredModel};

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
}
