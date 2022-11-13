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

mod database;

use actix_web::{HttpResponse, web};

pub fn mysql_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hello").route(web::get().to(hello)))
        .service(web::resource("/databases/create").route(web::post().to(database::create_database)))
        .service(web::resource("/databases/delete").route(web::post().to(database::drop_database)))
    ;
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, Mysql")
}
