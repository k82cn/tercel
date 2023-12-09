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
    v1alpha1::fabric::Fabric,
};
use yangtze_client::{YangtzeClient};

pub async fn run(client: YangtzeClient, kind: &String) -> Result<(), YangtzeError> {
    let client = client.version("v1alpha1").kind(kind);
    let fabric_list = client.list::<Fabric>(ALL).await?;

    println!(" {:<45}| {:<20}| {:<10}", "UUID", "Namespace", "Name",);
    for _ in 1..80 {
        print!("-");
    }
    println!();

    for f in fabric_list {
        println!(
            " {:<45}| {:<20}| {:<10}",
            f.meta_data.uuid.unwrap().to_string(),
            f.meta_data.namespace,
            f.meta_data.name
        );
    }

    Ok(())
}
