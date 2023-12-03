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

use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgRow;
use sqlx::query_builder::QueryBuilder;

use sqlx::{FromRow, Pool, Postgres, Row};
use uuid::Uuid;

use crate::storage::{Object, Storage};
use yangtze_apis::v1::{Metadata, YangtzeError};

pub struct PostgresStorage {
    pool: Pool<Postgres>,
}

impl PostgresStorage {
    pub async fn new(url: String) -> Result<Self, YangtzeError> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(10)
                .connect(&url)
                .await
                .map_err(|e| YangtzeError::GeneralError(e.to_string()))?,
        })
    }
}

#[async_trait]
impl Storage for PostgresStorage {
    async fn get(&self, id: String) -> Result<Object, YangtzeError> {
        let query = "SELECT * FROM objects WHERE id=$1";

        let uid = uuid::Uuid::parse_str(id.as_str())
            .map_err(|e| YangtzeError::GeneralError(e.to_string()))?;

        let obj: Object = sqlx::query_as(query)
            .bind(uid)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| YangtzeError::GeneralError(e.to_string()))?;

        return Ok(obj);
    }

    async fn list(&self, meta: Metadata) -> Result<Vec<Object>, YangtzeError> {
        let mut query = QueryBuilder::new("SELECT * FROM objects WHERE 1=1");

        if meta.uuid.is_some() {
            query.push(" AND id= ");
            query.push_bind(meta.uuid);
        }

        if !meta.kind.is_empty() {
            query.push(" AND kind= ");
            query.push_bind(meta.kind);
        }

        if !meta.namespace.is_empty() {
            query.push(" AND namespace= ");
            query.push_bind(meta.namespace);
        }

        if !meta.name.is_empty() {
            query.push(" AND name= ");
            query.push_bind(meta.name);
        }

        let obj = query
            .build()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| YangtzeError::GeneralError(e.to_string()))?
            .iter()
            .map(Object::from_row)
            .map_while(Result::ok)
            .collect();

        return Ok(obj);
    }

    async fn delete(&self, id: String) -> Result<Object, YangtzeError> {
        let query = "DELETE FROM objects WHERE id=$1 RETURNING *";

        let uid = uuid::Uuid::parse_str(id.as_str())
            .map_err(|e| YangtzeError::GeneralError(e.to_string()))?;

        let obj: Object = sqlx::query_as(query)
            .bind(uid)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| YangtzeError::GeneralError(e.to_string()))?;

        return Ok(obj);
    }

    async fn create(&self, o: Object) -> Result<Object, YangtzeError> {
        let uid = match o.metadata.uuid {
            Some(id) => id,
            None => Uuid::new_v4(),
        };

        let query = "INSERT INTO objects (
            id,
            kind,
            namespace,
            name,
            version,
            spec,
            status)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *";

        let obj: Object = sqlx::query_as(query)
            .bind(uid)
            .bind(&o.metadata.kind)
            .bind(&o.metadata.namespace)
            .bind(&o.metadata.name)
            .bind(o.metadata.version)
            .bind(&o.spec)
            .bind(&o.status)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| YangtzeError::GeneralError(e.to_string()))?;

        return Ok(obj);
    }

    async fn update(&self, o: Object) -> Result<Object, YangtzeError> {
        let query = "UPDATE objects SET spec=$3, status=$4, version=version+1 WHERE id=$1 AND version <= $2 RETURNING *";

        let obj: Object = sqlx::query_as(query)
            .bind(o.metadata.uuid)
            .bind(o.metadata.version)
            .bind(&o.spec)
            .bind(&o.status)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| YangtzeError::GeneralError(e.to_string()))?;

        return Ok(obj);
    }
}

impl<'r> FromRow<'r, PgRow> for Object {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Object {
            metadata: Metadata {
                uuid: Some(row.try_get("id")?),
                kind: row.try_get("kind")?,
                namespace: row.try_get("namespace")?,
                name: row.try_get("name")?,
                version: row.try_get("version")?,
                labels: vec![],
            },
            spec: row.try_get("spec")?,
            status: row.try_get("status")?,
        })
    }
}
