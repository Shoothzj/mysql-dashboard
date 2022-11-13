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

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MYSQL_HOST: String = std::env::var("MYSQL_HOST").unwrap_or_else(|_| "localhost".to_string());
    pub static ref MYSQL_PORT: u16 = std::env::var("MYSQL_PORT").unwrap_or_else(|_| "3306".to_string()).parse().unwrap();
    pub static ref MYSQL_USERNAME: String = std::env::var("MYSQL_USERNAME").unwrap_or_else(|_| "hzj".to_string());
    pub static ref MYSQL_PASSWORD: String = std::env::var("MYSQL_PASSWORD").unwrap_or_else(|_| "Mysql@123".to_string());
    pub static ref MYSQL_URL: String = format!("mysql://{}:{}@{}:{}/", MYSQL_USERNAME.as_str(), MYSQL_PASSWORD.as_str(), MYSQL_HOST.as_str(), MYSQL_PORT.to_owned());
}
