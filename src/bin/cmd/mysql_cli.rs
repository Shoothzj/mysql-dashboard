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

use clap::{Args, Parser, Subcommand};

use mysql_dashboard::{mysql, util};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value = "localhost")]
    host: String,
    #[arg(long, default_value = "3306")]
    port: u16,
    #[arg(long, default_value = "hzj")]
    user: String,
    #[arg(long, default_value = "Mysql@123")]
    password: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Msql tables command
    #[clap(name = "tables")]
    Tables(Tables),
}

#[derive(Args)]
struct Tables {
    #[command(subcommand)]
    command: TablesCommands,
    #[arg(long, default_value = "ttbb")]
    database: String,
}

#[derive(Subcommand)]
enum TablesCommands {
    /// Msql tables command
    #[clap(name = "clean")]
    TablesClean(TablesClean),
}

#[derive(Args)]
struct TablesClean {
    #[arg(long)]
    tables: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    util::init();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Tables(tables) => match &tables.command {
            TablesCommands::TablesClean(tables_clean) => {
                mysql::mysql_client::new(cli.host, cli.port, cli.user, cli.password, tables.database.clone())
                    .drop_tables(tables.database.clone(), mysql_dashboard::mysql::mysql_client::DropTableListReq { table_list: tables_clean.tables.clone() })
                    .await?;
            }
        },
    }
    Ok(())
}
