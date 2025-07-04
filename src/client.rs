use anyhow::Result;
use reqwest::Client;

use crate::RegisteredModelSearchResult;

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

        let response = self
            .client
            .get(url)
            .query(&[("filter", format!("name LIKE '{sql_pattern}'"))])
            .send()
            .await?;

        match response.json::<RegisteredModelSearchResult>().await {
            Ok(models) => {
                for model in models.registered_models {
                    println!("{}", model.name);
                }
            }
            Err(_) => println!("No model found with that pattern"),
        }
        Ok(())
    }
}
