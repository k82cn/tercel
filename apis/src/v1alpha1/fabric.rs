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

use std::fmt::{self, Display, Formatter};

use crate::v1::{Metadata, VersionKind};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FabricState {
    Initializing,
    Ready,
    Error,
    Deleting,
    Deleted,
}

impl fmt::Display for FabricState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FabricState::Initializing => write!(f, "Initializing"),
            FabricState::Ready => write!(f, "Ready"),
            FabricState::Error => write!(f, "Error"),
            FabricState::Deleting => write!(f, "Deleting"),
            FabricState::Deleted => write!(f, "Deleted"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FabricSpec {
    pub selector: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FabricStatus {
    pub state: FabricState,
    #[serde(default)]
    pub total: u64,
    #[serde(default)]
    pub available: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fabric {
    pub meta_data: Metadata,
    pub spec: FabricSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<FabricStatus>,
}

impl Display for Fabric {
    fn fmt(&self, writer: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(writer, "{0}", self.meta_data.name)
    }
}

pub const VERSION_KIND: VersionKind = VersionKind {
    version: "v1alpha1",
    kind: "fabric",
};
