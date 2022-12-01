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

use crate::mysql::mysql_client;
use crate::mysql::mysql_client::{CreateDatabaseReq, DropDatabaseReq};

pub async fn create_database(req: web::Json<CreateDatabaseReq>) -> HttpResponse {
    match mysql_client::from_env().create_database(req.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

pub async fn drop_database(req: web::Json<DropDatabaseReq>) -> HttpResponse {
    match mysql_client::from_env().drop_database(req.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}
