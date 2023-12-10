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

use std::{thread, time};

use async_trait::async_trait;

use serde::de::DeserializeOwned;

use tokio::task::JoinSet;
use yangtze_apis::v1::{VersionKind, YangtzeError, ALL};
use yangtze_client::{YangtzeClient, YangtzeConfig};

#[async_trait]
pub trait Controller<T: DeserializeOwned + Send + Sync + 'static>: Send + Sync {
    async fn execute(&self, client: YangtzeClient, t: T) -> Result<(), YangtzeError>;

    fn get_version_kind(&self) -> VersionKind;

    async fn run(&self, config: &YangtzeConfig) -> Result<(), YangtzeError> {
        let vk = self.get_version_kind();
        let client = YangtzeClient::new(config)?
            .version(vk.version)
            .kind(vk.kind);

        tracing::info!("A controller for <{}> was started.", vk);

        loop {
            thread::sleep(time::Duration::from_secs(10));
            let os = client.list::<T>(ALL).await?;
            for o in os {
                if let Err(e) = self.execute(client.clone(), o).await {
                    tracing::error!("{}", e.to_string());
                }
            }
        }

        // tracing::info!("The controller of <{}> was stopped.", vk);
    }
}

pub fn runtime(config: YangtzeConfig) -> Runtime {
    Runtime {
        config,
        futures: JoinSet::new(),
    }
}

pub struct Runtime {
    config: YangtzeConfig,
    futures: JoinSet<()>,
}

impl Runtime {
    pub async fn register<C, T>(mut self, c: C) -> Self
    where
        T: DeserializeOwned + Send + Sync + 'static,
        C: Controller<T> + 'static,
    {
        let config = self.config.clone();
        self.futures.spawn(async move {
            let _ = c.run(&config).await;
        });

        self
    }

    pub async fn run(mut self) {
        while (self.futures.join_next().await).is_some() {}
    }
}
