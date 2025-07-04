mod client;
mod models;

use anyhow::Result;
use clap::{Parser, Subcommand};

use client::MlflowClient;

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
    #[command(about = "List model versions")]
    ListVersions {
        #[arg()]
        model_name: String,
    },
    #[command(about = "Get model version by alias")]
    GetAlias {
        #[arg()]
        model_name: String,
        #[arg(short, long, help = "Model alias")]
        alias: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mlflow_tracking_uri = std::env::var("MLFLOW_TRACKING_URI")
        .unwrap_or("http://localhost:5000".to_string())
        .trim_end_matches('/')
        .to_string();

    let client = MlflowClient::new(mlflow_tracking_uri);

    match args.cmd {
        Commands::ListModels { pattern } => client.list_models(&pattern).await?,
        Commands::ListVersions { model_name } => client.list_versions(&model_name).await?,
        Commands::GetAlias { model_name, alias } => {
            client.get_version_by_alias(&model_name, &alias).await?
        }
    };

    Ok(())
}
