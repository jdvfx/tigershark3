use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

mod assetdef;
pub mod errors;
pub mod utils;

pub mod parse_args;
use errors::{exit_or_panic, CliOutput, TigerSharkError};
use parse_args::{AssetJson, CommandType};

#[tokio::main]
async fn main() {
    // get database_url from .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let cli_output: CliOutput;
    // parse args
    let args = parse_args::get_args();
    match args {
        Ok(args) => {
            //
            let options = SqlitePoolOptions::new().max_connections(5);
            let pool = SqlitePoolOptions::connect(options, &database_url).await;
            if pool.is_err() {
                let cli_output = CliOutput(Err(TigerSharkError::DbError(format!(
                    "Could not connect to database"
                ))));
                exit_or_panic(cli_output);
            }

            // connect to DB
            let conn = pool.unwrap().acquire().await;
            match conn {
                Ok(conn) => {
                    let json = args.json.unwrap_or_default();
                    cli_output = match args.command {
                        CommandType::Insert => utils::insert(conn, json).await,
                        CommandType::Source => utils::source(conn, json).await,
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
