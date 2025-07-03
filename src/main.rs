mod models;

use anyhow::Result;
use clap::{Parser, Subcommand};
use models::RegisteredModelSearchResult;
use reqwest::Client;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "List registered models")]
    ListModels {
        #[arg()]
        pattern: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mlflow_tracking_uri =
        std::env::var("MLFLOW_TRACKING_URI").unwrap_or("http://localhost:5000".to_string());
    let client = Client::new();

    match args.cmd {
        Commands::ListModels { pattern } => {
            let url = format!("{mlflow_tracking_uri}/api/2.0/mlflow/registered-models/search");
            let sql_pattern = if pattern.contains('*') {
                pattern.replace("*", "%")
            } else {
                format!("%{pattern}%")
            };
            let response = client
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
        }
    }

    Ok(())
}
