use clap::Parser;
use serde::{Deserialize, Serialize};

pub use crate::assetdef::Asset;
use crate::errors::CliOutput;

#[derive(Debug)]
pub enum CommandType {
    Insert,
    Source,
    Delete,
    Latest,
    Approve,
}

#[derive(Debug)]
pub struct Command {
    pub command: CommandType,
    pub json: AssetJson,
}

/// Parse Command and Asset(json) arguments
#[derive(Parser, Debug)]
#[clap(author="Julien D.", version, about, long_about = None)]
struct Args {
    /// CRUD command
    #[clap(short, long, value_parser)]
    command: String,

    /// json string representing the asset
    #[clap(short, long, value_parser)]
    asset: String,
}

// serialized by Serde (could have missing fields: Options)
#[derive(Debug, Serialize, Deserialize)]
struct JsonOption {
    pub asset_id: Option<i64>,
    pub name: Option<String>,
    pub location: Option<String>,
    pub version_id: Option<i64>,
    pub version: Option<i64>,
    pub source: Option<String>,
    pub datapath: Option<String>,
    pub depend: Option<String>,
    pub approved: Option<u8>,
    pub status: Option<u8>,
}
// the asset json that gets passed to the CRUD function
#[derive(Debug, Clone)]
pub struct AssetJson {
    pub asset_id: i64,
    pub name: String,
    pub location: String,
    pub version_id: i64,
    pub version: i64,
    pub source: String,
    pub datapath: String,
    pub depend: String,
    pub approved: u8,
    pub status: u8,
}
// create default empty values if missing
// removes the need for unwrap() when executing CRUD commands
fn json_unwrap_or(json_o: JsonOption) -> AssetJson {
    AssetJson {
        asset_id: json_o.asset_id.unwrap_or(0),
        name: json_o.name.unwrap_or_else(|| "".to_owned()),
        location: json_o.location.unwrap_or_else(|| "".to_owned()),
        version_id: json_o.version_id.unwrap_or(0),
        version: json_o.version.unwrap_or(0),
        source: json_o.source.unwrap_or_else(|| "".to_owned()),
        datapath: json_o.datapath.unwrap_or_else(|| "".to_owned()),
        depend: json_o.depend.unwrap_or_else(|| "".to_owned()),
        approved: json_o.approved.unwrap_or(0),
        status: json_o.status.unwrap_or(0),
    }
}

// pub fn get_args() -> Option<Command> {
pub fn get_args() -> Result<Command, CliOutput> {
    //
    let args = Args::parse();

    // >>> ASSET ---
    // Asset is defined in assetdef.rs
    // get asset String from args and try to parse using struct above
    let asset_str = args.asset.to_string();
    let asset_result: serde_json::Result<JsonOption> = serde_json::from_str(&asset_str);
    let asset: JsonOption = match asset_result {
        Ok(a) => a,
        Err(r) => {
            return Err(CliOutput::new(
                "err",
                &format!("Err: bad json format: {} : {:?}", asset_str, r),
            ))
        }
    };
    // to check if json values are present for the current command
    let a_name = asset.name.is_some();
    let a_location = asset.location.is_some();
    let asset_id = asset.asset_id.is_some();
    let a_version = asset.version.is_some();
    let a_version_id = asset.version_id.is_some();

    // unpack JsonOption into JsonString
    let asset_unwrapped: AssetJson = json_unwrap_or(asset);

    // >>> COMMAND <<<
    // for each command, checks that the correct json values are present
    match args.command.as_str() {
        "insert" => match a_name && a_location || asset_id || a_version_id {
            // source and datapath are optional => update asset
            // otherwize, just create a new asset if needed
            true => Ok(Command {
                command: CommandType::Insert,
                json: asset_unwrapped,
            }),
            _ => Err(CliOutput::new("err", "create : Asset missing some Keys")),
        },
        "source" => match (a_name && a_location || asset_id) && a_version || a_version_id {
            true => Ok(Command {
                command: CommandType::Source,
                json: asset_unwrapped,
            }),
            _ => Err(CliOutput::new("err", "source : Asset missing some Keys")),
        },
        "delete" => match (a_name && a_location || asset_id) && a_version || a_version_id {
            true => Ok(Command {
                command: CommandType::Delete,
                json: asset_unwrapped,
            }),
            _ => Err(CliOutput::new("err", "delete : Asset missing some Keys")),
        },
        "latest" => match a_name && a_location || asset_id {
            true => Ok(Command {
                command: CommandType::Latest,
                json: asset_unwrapped,
            }),
            _ => Err(CliOutput::new("err", "latest : Asset missing some Keys")),
        },
        "approve" => match (a_name && a_location || asset_id) && a_version || a_version_id {
            true => Ok(Command {
                command: CommandType::Approve,
                json: asset_unwrapped,
            }),
            _ => Err(CliOutput::new("err", "approve : Asset missing some Keys")),
        },
        _ => Err(CliOutput::new("err", "invalid a command")),
    }
}
