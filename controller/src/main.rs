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

use yangtze_apis::v1::YangtzeError;
use yangtze_client::YangtzeConfig;

mod fabrics;
mod framework;
mod switches;

#[tokio::main]
async fn main() -> Result<(), YangtzeError> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| YangtzeError::GeneralError(e.to_string()))?;

    let config = YangtzeConfig {
        address: "http://127.0.0.1:8080".to_string(),
    };

    let mut rt = framework::runtime(config);

    rt = rt.register(fabrics::FabricController {}).await;
    rt = rt.register(switches::SwitchController {}).await;

    rt.run().await;

    Ok(())
}
