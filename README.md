# mmr - MLflow Model Registry CLI

A command-line tool for interacting with MLflow Model Registry.

## Features

- **List registered models** with optional pattern matching
- **List model versions** for a specific model
- **Get model version by alias**

## Installation

Download latest build binaries or build from source

```bash
cargo build --release
```

## Usage

Set your MLflow tracking URI:

```bash
# default: `http://localhost:5000`
export MLFLOW_TRACKING_URI=http://your-mlflow-server:5000
```

### List Models

```bash
# List all models
mmr list-models

# List models with pattern (supports wildcards)
mmr list-models "my-model*"

# Alias for list-models
mmr ls
```

### List Model Versions

```bash
# List all versions for a model
mmr list-versions "my-model"
```

### Get Model by Alias

```bash
# Get specific model version by alias
mmr get-alias --alias "production" "my-model"
```

