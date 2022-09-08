#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use sqlx::sqlite;
use sqlx::Connection;

use crate::assetdef::Asset;
mod assetdef;
pub mod errors;
pub mod utils;

pub mod parse_args;
use errors::{exit_or_panic, CliOutput};
use parse_args::CommandType;
//

#[tokio::main]
async fn main() {
    let cli_output: CliOutput;
    // parse args
    let args = parse_args::get_args();
    match args {
        Ok(args) => {
            //
            let db_name = "sqlite:/home/bunker/assets2.db";
            let conn = sqlite::SqliteConnection::connect(&db_name).await;
            // connect to db, return a connection
            match conn {
                Ok(conn) => {
                    // Get asset json (eg: -a '{"name":"box",...}')
                    let json = args.json;
                    // Get command (eg: -c create )
                    // Execute one of the commands
                    cli_output = match args.command {
                        CommandType::Create => utils::create(conn, json).await,
                        CommandType::Update => utils::update(conn, json).await,
                        CommandType::Source => utils::source(conn, json).await,
                        CommandType::Delete => utils::delete(conn, json).await,
                        CommandType::Latest => utils::latest(conn, json).await,
                        CommandType::Approve => utils::approve(conn, json).await,
                    };
                }
                Err(e) => {
                    cli_output =
                        CliOutput::new("err", &format!("Error with the connection: {:?}", e));
                }
            }
        }
        Err(o) => cli_output = CliOutput::new("err", &format!("Error parsing args: {:?}", o)),
    }
    exit_or_panic(cli_output);
}

// #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {
//     //
//     let db_name = "sqlite:/home/bunker/assets2.db";
//     //
//     // create_assets_table(&db_name).await?;
//     // create_versions_table(&db_name).await?;
//     // insert_version(&db_name).await?;
//     // insert_asset(&db_name, "my_new_asset").await?;
//     find_asset_id(&db_name, "cone").await?;
//     //
//     Ok(())
// }
