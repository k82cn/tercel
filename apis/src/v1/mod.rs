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

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use tracing::error;

use actix_web::{error, http::StatusCode, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    pub kind: String,
    pub namespace: String,
    pub name: String,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub version: i32,
}

impl Display for Metadata {
    fn fmt(&self, writer: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(writer, "{0}/{1}/{2}", self.kind, self.namespace, self.name)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum YangtzeError {
    #[error("{0}")]
    GeneralError(String),
    #[error("{0}")]
    RestfulError(String),
    #[error("{0}")]
    InvalidConfig(String),
}

impl From<serde_json::Error> for YangtzeError {
    fn from(e: serde_json::Error) -> YangtzeError {
        YangtzeError::GeneralError(e.to_string())
    }
}

impl error::ResponseError for YangtzeError {
    fn status_code(&self) -> StatusCode {
        error!("{}", self.to_string());
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NamespaceName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

pub const ALL: NamespaceName = NamespaceName {
    namespace: None,
    name: None,
};
