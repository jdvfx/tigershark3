use sqlx::sqlite;
use sqlx::Connection;

// use crate::assetdef::Asset;
mod assetdef;
pub mod errors;
pub mod utils;

pub mod parse_args;
use errors::{exit_or_panic, CliOutput};
use parse_args::CommandType;
//

#[tokio::main]
async fn main() {
    //
    let initialize_tables = false;
    if initialize_tables {
        let db_name = "sqlite:/home/bunker/assets2.db";
        let conn = sqlite::SqliteConnection::connect(&db_name).await;
        match conn {
            Ok(c) => {
                let _a = utils::create_versions_table(c).await;
                println!("OK : Versions table created");
            }
            Err(e) => {
                println!("ERR {:?}", e);
            }
        }
        let conn = sqlite::SqliteConnection::connect(&db_name).await;
        match conn {
            Ok(c) => {
                let _a = utils::create_asset_table(c).await;
                println!("OK : Assets table created");
            }
            Err(e) => {
                println!("ERR {:?}", e);
            }
        }
        panic!("Created Assets and Versions tables");
    }

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
        Err(e) => cli_output = CliOutput::new("err", &format!("Error parsing args: {:?}", e)),
    }
    exit_or_panic(cli_output);
}
