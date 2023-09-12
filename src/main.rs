#![forbid(unsafe_code)]
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

mod assetdef;
pub mod errors;
pub mod utils;

pub mod frame_num;
pub mod parse_args;

// use crate::testo::test_frame_nums::*;

use errors::{exit_or_panic, CliOutput, TigerSharkError};
use parse_args::CommandType;

#[tokio::main]
async fn main() {
    // get database_url from .env file
    dotenv().ok();
    let db_url = env::var("TS_DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = format!("sqlite:{db_url}");

    let cli_output: CliOutput;
    // parse args
    let args = parse_args::get_args();
    match args {
        Ok(args) => {
            //
            let options = SqlitePoolOptions::new().max_connections(5);
            let pool = SqlitePoolOptions::connect(options, &database_url).await;
            if pool.is_err() {
                let cli_output = CliOutput(Err(TigerSharkError::DbError(
                    "Could not connect to database".to_owned(),
                )));
                exit_or_panic(cli_output);
            }

            // connect to DB
            let conn = pool.unwrap().acquire().await;
            match conn {
                Ok(conn) => {
                    let json = args.json.unwrap_or_default();
                    let file = args.file.unwrap_or_default();
                    cli_output = match args.command {
                        CommandType::Insert => utils::insert(conn, json).await,
                        CommandType::Source => utils::source(conn, json).await,
                        CommandType::SourceFromFile => utils::source_from_file(conn, file).await,
                        CommandType::Delete => utils::delete(conn, json).await,
                        CommandType::Latest => utils::latest(conn, json).await,
                        CommandType::Approve => utils::approve(conn, json).await,
                        CommandType::Purge => utils::purge(conn).await,
                    };
                }
                Err(e) => {
                    cli_output = CliOutput(Err(TigerSharkError::DbError(format!(
                        "connection error: {e}"
                    ))));
                }
            }
        }
        Err(e) => {
            cli_output = CliOutput(Err(TigerSharkError::CliError(format!(
                "Error parsing args: {e:?}"
            ))))
        }
    }
    exit_or_panic(cli_output);
}
