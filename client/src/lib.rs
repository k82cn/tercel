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

use std::fmt;
use std::fmt::{Display, Formatter};
use url::Url;

use http_body_util::{BodyExt, Empty, Full};
use hyper::body::Bytes;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::http::StatusCode;
use hyper::{body::Buf, Request};
use hyper::{Method, Response};
use hyper_tls::HttpsConnector;
use hyper_util::rt::TokioIo;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use tokio::net::TcpStream;

use yangtze_apis::v1::{NamespaceName, YangtzeError};

#[derive(Clone)]
pub struct YangtzeConfig {
    pub address: String,
}

#[derive(Clone)]
pub struct YangtzeClient {
    address: String,
    version: Option<String>,
    kind: Option<String>,
}

impl YangtzeClient {
    pub fn new(config: &YangtzeConfig) -> Result<Self, YangtzeError> {
        let url = Url::parse(&config.address).map_err(|_| {
            YangtzeError::InvalidConfig("invalid yangtze-apiserver url".to_string())
        })?;
        let host = url.host_str().ok_or(YangtzeError::InvalidConfig(
            "invalid yangtze-apiserver host".to_string(),
        ))?;
        let port = url.port().unwrap_or(80);
        let address = format!("{}:{}", host, port);

        Ok(YangtzeClient {
            address,
            version: None,
            kind: None,
        })
    }

    pub fn version(mut self, v: &str) -> Self {
        self.version = Some(v.to_string());

        self
    }

    pub fn kind(mut self, k: &str) -> Self {
        self.kind = Some(k.to_string());

        self
    }

    pub async fn get<T: DeserializeOwned>(&self, id: String) -> Result<T, YangtzeError> {
        let body = self
            .execute_request::<T>(Method::GET, Some(id), None)
            .await?;
        serde_json::from_reader(body.reader())
            .map_err(|e| YangtzeError::RestfulError(e.to_string()))
    }

    pub async fn list<T: DeserializeOwned>(
        &self,
        nn: NamespaceName,
    ) -> Result<Vec<T>, YangtzeError> {
        let input = serde_json::to_string(&nn)?;
        let body = self
            .execute_request::<T>(Method::POST, None, Some(input))
            .await?;

        serde_json::from_reader(body.reader())
            .map_err(|e| YangtzeError::RestfulError(e.to_string()))
    }

    pub async fn create<T: DeserializeOwned + Serialize>(&self, o: T) -> Result<T, YangtzeError> {
        let input = serde_json::to_string(&o)?;
        let body = self
            .execute_request::<T>(Method::PUT, None, Some(input))
            .await?;

        serde_json::from_reader(body.reader())
            .map_err(|e| YangtzeError::RestfulError(e.to_string()))
    }

    pub async fn delete<T: DeserializeOwned>(&self, id: String) -> Result<T, YangtzeError> {
        let body = self
            .execute_request::<T>(Method::DELETE, Some(id), None)
            .await?;
        serde_json::from_reader(body.reader())
            .map_err(|e| YangtzeError::RestfulError(e.to_string()))
    }

    pub async fn update<T: DeserializeOwned + Serialize>(&self, o: T) -> Result<T, YangtzeError> {
        let input = serde_json::to_string(&o)?;
        let body = self
            .execute_request::<T>(Method::PATCH, None, Some(input))
            .await?;

        serde_json::from_reader(body.reader())
            .map_err(|e| YangtzeError::RestfulError(e.to_string()))
    }

    fn base_url(&self) -> String {
        let mut url = String::new();

        if let Some(v) = self.version.clone() {
            url = v;
        }

        if let Some(k) = self.kind.clone() {
            url = format!("{}/{}", url, k);
        }

        url
    }

    async fn execute_request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: Option<String>,
        data: Option<String>,
    ) -> Result<Bytes, YangtzeError> {
        let schema = "http";
        let url = match path {
            Some(p) => format!(
                "{}://{}/{}/{}",
                schema,
                self.address,
                self.base_url(),
                p.trim_matches('/')
            ),
            None => format!("{}://{}/{}", schema, self.address, self.base_url()),
        };

        let body = data.unwrap_or(String::new());

        let req = hyper::Request::builder()
            .method(method)
            .uri(url)
            .header(CONTENT_TYPE, "application/json")
            // .header(AUTHORIZATION, self.auth_info.to_string())
            .body(Full::<Bytes>::new(Bytes::from(body)))
            .map_err(|e| YangtzeError::InvalidConfig(e.to_string()))?;

        let stream = TcpStream::connect(self.address.clone())
            .await
            .map_err(|e| YangtzeError::RestfulError(e.to_string()))?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io)
            .await
            .map_err(|e| YangtzeError::RestfulError(e.to_string()))?;

        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                tracing::error!("Failed to connect to yangtze-apiserver: {:?}", err);
            }
        });

        let resp = sender
            .send_request(req)
            .await
            .map_err(|e| YangtzeError::RestfulError(e.to_string()))?;

        if resp.status() != StatusCode::OK {
            return Err(YangtzeError::RestfulError(format!("{}", resp.status())));
        }

        let body = resp
            .collect()
            .await
            .map_err(|e| YangtzeError::RestfulError(e.to_string()))?;

        Ok(body.to_bytes())
    }
}
