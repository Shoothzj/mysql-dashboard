// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use actix_web::{HttpResponse, web};
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;
use crate::constant;

#[derive(Deserialize)]
pub struct CreateDatabaseReq {
    database_name: String,
}

pub async fn create_database(req: web::Json<CreateDatabaseReq>) -> HttpResponse {
    match create_database_internal(req.0).await {
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn create_database_internal(req: CreateDatabaseReq) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::new(constant::MYSQL_URL.as_str())?;
    let mut conn = pool.get_conn()?;
    conn.query_drop(format!("CREATE DATABASE {}", req.database_name))?;
    Ok(())
}

#[derive(Deserialize)]
pub struct DropDatabaseReq {
    database_name: String,
}

pub async fn drop_database(req: web::Json<DropDatabaseReq>) -> HttpResponse {
    match drop_database_internal(req.0).await {
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn drop_database_internal(req: DropDatabaseReq) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::new(constant::MYSQL_URL.as_str())?;
    let mut conn = pool.get_conn()?;
    conn.query_drop(format!("DROP DATABASE {}", req.database_name))?;
    Ok(())
}
