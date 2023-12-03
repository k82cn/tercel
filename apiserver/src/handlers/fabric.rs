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
use actix_web::{delete, get, patch, post, put, web, Responder};
use std::sync::Arc;


use yangtze_apis::{
    v1::Metadata,
    v1alpha1::fabric::{Fabric, FabricState, FabricStatus},
};

use crate::storage::{Object, Storage};
use yangtze_apis::v1::YangtzeError;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/v1alpha1")
        .service(get)
        .service(list)
        .service(create)
        .service(delete)
        .service(update);

    conf.service(scope);
}

#[get("/fabric/{id}")]
pub async fn get(
    id: web::Path<String>,
    storage: web::Data<Arc<dyn Storage>>,
) -> actix_web::Result<impl Responder> {
    let obj = storage.get(id.to_string()).await?;
    let fabric = Fabric::try_from(obj)?;

    Ok(web::Json(fabric))
}

#[post("/fabric")]
pub async fn list(
    meta: web::Json<Metadata>,
    storage: web::Data<Arc<dyn Storage>>,
) -> actix_web::Result<impl Responder> {
    let obj = storage.list(meta.0).await?;
    let fabric: Vec<_> = obj
        .iter()
        .map(Fabric::try_from)
        .map_while(Result::ok)
        .collect();

    Ok(web::Json(fabric))
}

#[delete("/fabric/{id}")]
pub async fn delete(
    id: web::Path<String>,
    storage: web::Data<Arc<dyn Storage>>,
) -> actix_web::Result<impl Responder> {
    let obj = storage.delete(id.to_string()).await?;
    let fabric = Fabric::try_from(obj)?;

    Ok(web::Json(fabric))
}

#[put("/fabric")]
pub async fn create(
    fabric: web::Json<Fabric>,
    storage: web::Data<Arc<dyn Storage>>,
) -> actix_web::Result<impl Responder> {
    let fabric = Fabric {
        status: Some(FabricStatus {
            state: FabricState::Initializing,
            total: 0,
            available: 0,
        }),
        meta_data: fabric.0.meta_data,
        spec: fabric.0.spec,
    };
    let obj = Object::try_from(fabric)?;
    let obj = storage.create(obj).await?;
    let fabric = Fabric::try_from(obj)?;

    Ok(web::Json(fabric))
}

#[patch("/fabric")]
pub async fn update(
    fabric: web::Json<Fabric>,
    storage: web::Data<Arc<dyn Storage>>,
) -> actix_web::Result<impl Responder> {
    let obj = Object::try_from(fabric.0)?;
    let obj = storage.update(obj).await?;
    let fabric = Fabric::try_from(obj)?;

    Ok(web::Json(fabric))
}

impl TryFrom<Object> for Fabric {
    type Error = YangtzeError;

    fn try_from(o: Object) -> Result<Self, Self::Error> {
        Fabric::try_from(&o)
    }
}

impl TryFrom<&Object> for Fabric {
    type Error = YangtzeError;

    fn try_from(o: &Object) -> Result<Self, Self::Error> {
        Ok(Fabric {
            meta_data: o.metadata.clone(),
            spec: serde_json::from_str(o.spec.as_str())?,
            status: serde_json::from_str(o.status.as_str())?,
        })
    }
}

impl TryFrom<Fabric> for Object {
    type Error = YangtzeError;

    fn try_from(f: Fabric) -> Result<Self, Self::Error> {
        let _metadata = f.meta_data.clone();

        Ok(Object {
            metadata: f.meta_data.clone(),
            spec: serde_json::to_string(&f.spec)?,
            status: serde_json::to_string(&f.status)?,
        })
    }
}
