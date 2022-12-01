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

use mysql::prelude::Queryable;
use serde::Deserialize;

use crate::constant;

pub struct MysqlClient {
    pool: mysql::Pool,
}

pub fn from_env() -> MysqlClient {
    new_url(constant::MYSQL_URL.as_str())
}

pub fn new(host: String, port: u16, user: String, password: String, database: String) -> MysqlClient {
    let url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, database);
    new_url(url.as_str())
}

fn new_url(url: &str) -> MysqlClient {
    let client = mysql::Pool::new(url).unwrap();
    MysqlClient {
        pool: client,
    }
}

#[derive(Deserialize)]
pub struct CreateDatabaseReq {
    database_name: String,
}

#[derive(Deserialize)]
pub struct DropDatabaseReq {
    pub database_name: String,
}

#[derive(Deserialize)]
pub struct DropTableListReq {
    pub table_list: Vec<String>,
}

impl MysqlClient {
    pub async fn create_database(&self, req: CreateDatabaseReq) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.query_drop(format!("CREATE DATABASE {}", req.database_name))?;
        Ok(())
    }

    pub async fn drop_database(&self, req: DropDatabaseReq) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.query_drop(format!("DROP DATABASE {}", req.database_name))?;
        Ok(())
    }

    pub async fn drop_tables(&self, database_name: String, req: DropTableListReq) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.get_conn()?;
        for table_name in req.table_list {
            conn.query_drop(format!("DROP TABLE {}.{}", database_name, table_name))?;
        }
        Ok(())
    }
}
