/*
 * Copyright 2023 The xflops Authors.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *     http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use clap::{Parser, Subcommand};

use yangtze_apis::{
    v1::{YangtzeError},
};
use yangtze_client::{YangtzeClient, YangtzeConfig};

mod helper;
mod list;

#[derive(Parser)]
#[command(name = "yzctl")]
#[command(author = "Klaus Ma <klaus@xflops.cn>")]
#[command(version = "0.1.0")]
#[command(about = "Yangtze command line", long_about = None)]
struct Cli {
    #[arg(long)]
    flame_conf: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List {
        #[arg(short, long)]
        kind: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), YangtzeError> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| YangtzeError::GeneralError(e.to_string()))?;

    let client = YangtzeClient::new(&YangtzeConfig {
        address: "http://127.0.0.1:8080".to_string(),
    })?;

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::List { kind }) => list::run(client, kind).await?,
        _ => helper::run().await?,
    };

    Ok(())
}
