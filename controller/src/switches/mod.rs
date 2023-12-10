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

use yangtze_apis::{
    v1::{VersionKind, YangtzeError},
    v1alpha1::fabric::{self, Fabric},
};
use yangtze_client::YangtzeClient;

use crate::framework::Controller;

#[derive(Clone)]
pub struct SwitchController {}

#[async_trait]
impl Controller<Fabric> for SwitchController {
    async fn execute(&self, _client: YangtzeClient, _f: Fabric) -> Result<(), YangtzeError> {
        tracing::info!("switch controller");
        Ok(())
    }

    fn get_version_kind(&self) -> VersionKind {
        fabric::VERSION_KIND.clone()
    }
}
