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

use yangtze_apis::{
    v1::{YangtzeError, ALL},
    v1alpha1::fabric::{Fabric, FabricState},
};
use yangtze_client::YangtzeClient;

pub async fn run(client: YangtzeClient, kind: &str) -> Result<(), YangtzeError> {
    let vk = yangtze_apis::get_version_kind(&kind.to_lowercase())
        .ok_or(YangtzeError::InvalidConfig("unknown kind".to_string()))?;
    let client = client.version(vk.version).kind(vk.kind);
    let fabric_list = client.list::<Fabric>(ALL).await?;

    println!(
        " {:<45}| {:<20}| {:<10}| {:<15}",
        "UUID", "Namespace", "Name", "State"
    );
    for _ in 1..100 {
        print!("-");
    }
    println!();

    for f in fabric_list {
        let state = match f.status {
            Some(s) => s.state,
            None => FabricState::Initializing,
        };

        println!(
            " {:<45}| {:<20}| {:<10}| {:<15}",
            f.meta_data.uuid.unwrap().to_string(),
            f.meta_data.namespace,
            f.meta_data.name,
            state,
        );
    }

    Ok(())
}
