#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use dotenv::dotenv;
use std::env;

use sqlx::sqlite::SqlitePoolOptions;

mod assetdef;
pub mod errors;
pub mod utils;

pub mod parse_args;
use errors::{exit_or_panic, CliOutput, TigerSharkError};
use parse_args::CommandType;

#[tokio::main]
async fn main() {
    // get database_url from .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // let cli_output: CliOutput;
    let cli_output: CliOutput;
    // parse args
    let args = parse_args::get_args();
    match args {
        Ok(args) => {
            //
            // not sure about the syntax there... why using SqlitePoolOptions twice, that's dumb.
            let options = SqlitePoolOptions::new().max_connections(5);
            let pool = SqlitePoolOptions::connect(options, &database_url)
                .await
                .unwrap();
            let conn = pool.acquire().await;
            // let conn = sqlite::SqliteConnection::connect(&db_name).await;
            // connect to db, return a connection
            match conn {
                Ok(conn) => {
                    // Get asset json (eg: -a '{"name":"box",...}')
                    let json = args.json;
                    //
                    // it's OK to unwrap() the json below,
                    // it has been already checked in json_unwrap_or()
                    //
                    // Execute one of the commands
                    cli_output = match args.command {
                        CommandType::Insert => utils::insert(conn, json.unwrap()).await,
                        CommandType::Source => utils::source(conn, json.unwrap()).await,
                        CommandType::Delete => utils::delete(conn, json.unwrap()).await,
                        CommandType::Latest => utils::latest(conn, json.unwrap()).await,
                        CommandType::Approve => utils::approve(conn, json.unwrap()).await,
                        CommandType::Purge => utils::purge(conn).await,
                    };
                }
                Err(e) => {
                    // TODO : create database if it doesn't exist
                    cli_output = CliOutput(Err(TigerSharkError::DbError(
                        "connection error".to_string(),
                    )));
                }
            }
        }
        Err(e) => {
            cli_output = CliOutput(Err(TigerSharkError::CliError(format!(
                "Error parsing args: {e:?}"
            ))))
        }
    }

    // let cli_out = CliOutput(Ok("GOOD".to_string()));
    exit_or_panic(cli_output);
}
