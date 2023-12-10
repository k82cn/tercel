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
    v1alpha1::fabric::{self, Fabric, FabricState, FabricStatus},
};
use yangtze_client::YangtzeClient;

use crate::framework::Controller;

#[derive(Clone)]
pub struct FabricController {}

#[async_trait]
impl Controller<Fabric> for FabricController {
    async fn execute(&self, client: YangtzeClient, f: Fabric) -> Result<(), YangtzeError> {
        if f.meta_data.uuid.is_none() {
            return Err(YangtzeError::InvalidConfig(format!(
                "The id of <{}> is none.",
                f
            )));
        }
        let mut f = client
            .get::<Fabric>(f.meta_data.uuid.unwrap().to_string())
            .await?;

        f.status = match f.status {
            Some(status) => Some(FabricStatus {
                state: FabricState::Ready,
                ..status
            }),
            None => Some(FabricStatus {
                state: FabricState::Initializing,
                total: 0,
                available: 0,
            }),
        };

        let _f = client.update::<Fabric>(f).await?;

        Ok(())
    }

    fn get_version_kind(&self) -> VersionKind {
        fabric::VERSION_KIND.clone()
    }
}
