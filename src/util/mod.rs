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

use std::env;

pub fn init() {
    let log_level = match env::var("LOG_LEVEL") {
        Ok(level) => {
            if level == "trace" {
                log::LevelFilter::Trace
            } else if level == "debug" {
                log::LevelFilter::Debug
            } else if level == "info" {
                log::LevelFilter::Info
            } else if level == "warn" {
                log::LevelFilter::Warn
            } else if level == "error" {
                log::LevelFilter::Error
            } else {
                log::LevelFilter::Info
            }
        }
        Err(..) => {
            log::LevelFilter::Info
        }
    };
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply().unwrap();
}
