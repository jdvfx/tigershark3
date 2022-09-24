// use sqlx::sqlite;
use sqlx::sqlite::SqlitePoolOptions;
// use sqlx::{Acquire, Connection};

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
    let db_name = "sqlite:/home/bunker/assets2.db";
    let cli_output: CliOutput;
    // parse args
    let args = parse_args::get_args();
    match args {
        Ok(args) => {
            //
            // not sure about the syntax there... why using SqlitePoolOptions twice, that's dumb.
            let options = SqlitePoolOptions::new().max_connections(5);
            let pool = SqlitePoolOptions::connect(options, db_name).await.unwrap();
            let conn = pool.acquire().await;
            // let conn = sqlite::SqliteConnection::connect(&db_name).await;
            // connect to db, return a connection
            match conn {
                Ok(conn) => {
                    // Get asset json (eg: -a '{"name":"box",...}')
                    let json = args.json;
                    // Get command (eg: -c create )
                    // Execute one of the commands
                    cli_output = match args.command {
                        CommandType::Insert => utils::insert(conn, json).await,
                        CommandType::Source => utils::source(conn, json).await,
                        CommandType::Delete => utils::delete(conn, json).await,
                        CommandType::Latest => utils::latest(conn, json).await,
                        CommandType::Approve => utils::approve(conn, json).await,
                        CommandType::Test => utils::test(conn, json).await,
                        CommandType::Initialize => utils::initialize(conn).await,
                    };
                }
                Err(e) => {
                    // TODO : create database if it doesn't exist
                    cli_output =
                        CliOutput::new("err", &format!("Error with the connection: {:?}", e));
                }
            }
        }
        Err(e) => cli_output = CliOutput::new("err", &format!("Error parsing args: {:?}", e)),
    }
    exit_or_panic(cli_output);
}
