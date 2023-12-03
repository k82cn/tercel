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

use async_trait::async_trait;
use std::fmt::{Display, Formatter};
use std::sync::Arc;


use yangtze_apis::v1::{Metadata, YangtzeError};

mod db;

pub struct Object {
    pub metadata: Metadata,
    pub spec: String,
    pub status: String,
}

#[async_trait]
pub trait Storage: Send + Sync + 'static {
    async fn get(&self, id: String) -> Result<Object, YangtzeError>;
    async fn list(&self, meta: Metadata) -> Result<Vec<Object>, YangtzeError>;
    async fn delete(&self, id: String) -> Result<Object, YangtzeError>;
    async fn create(&self, o: Object) -> Result<Object, YangtzeError>;
    async fn update(&self, o: Object) -> Result<Object, YangtzeError>;
}

pub async fn new() -> Result<Arc<dyn Storage>, YangtzeError> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Ok(Arc::new(db::PostgresStorage::new(database_url).await?))
}

impl Display for Object {
    fn fmt(&self, writer: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            writer,
            "{0}/{1}/{2}",
            self.metadata.kind, self.metadata.namespace, self.metadata.name
        )
    }
}
