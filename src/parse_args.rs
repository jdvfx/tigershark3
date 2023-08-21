use clap::Parser;
use serde::{Deserialize, Serialize};

pub use crate::assetdef::Asset;
use crate::errors::CliOutput;

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum CommandType {
    Insert,
    Source,
    Delete,
    Latest,
    Approve,
    Purge,
}

#[derive(Debug)]
pub struct Command {
    pub command: CommandType,
    pub json: Option<AssetJson>,
    pub extra_args: Option<String>,
}

/// Parse Command and Asset(json) arguments
#[derive(Parser, Debug, PartialEq)]
#[clap(author="Julien D.", version, about, long_about = None)]
pub struct Args {
    /// CRUD command
    #[clap(short, long, value_enum)]
    pub command: CommandType,
    /// Json string representing the asset
    #[clap(short, long, value_parser)]
    pub asset: Option<String>,
    /// extra args to some commands
    #[clap(short, long, value_parser)]
    pub extra_args: Option<String>,
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
#[derive(Debug, Clone, Default)]
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
impl From<JsonOption> for AssetJson {
    fn from(json_o: JsonOption) -> AssetJson {
        AssetJson {
            asset_id: json_o.asset_id.unwrap_or_default(),
            name: json_o.name.unwrap_or_default(),
            location: json_o.location.unwrap_or_default(),
            version_id: json_o.version_id.unwrap_or_default(),
            version: json_o.version.unwrap_or_default(),
            source: json_o.source.unwrap_or_default(),
            datapath: json_o.datapath.unwrap_or_default(),
            depend: json_o.depend.unwrap_or_default(),
            approved: json_o.approved.unwrap_or_default(),
            status: json_o.status.unwrap_or_default(),
        }
    }
}

pub fn get_args() -> Result<Command, CliOutput> {
    //
    let args = Args::parse();
    // check that asset exists for all commands except Purge
    match args.command {
        CommandType::Purge => (),
        _ => {
            if args.asset.is_none() {
                return Err(CliOutput(Err(crate::errors::TigerSharkError::CliError(
                    format!("Requires Asset for function: {:?}", args.command),
                ))));
            }
        }
    }
    // >>> ASSET ---
    // Asset is defined in assetdef.rs
    // get asset String from args and try to parse using struct above
    let asset_str = args.asset.unwrap_or_else(|| "{}".to_owned());
    let asset_option: JsonOption = match serde_json::from_str(&asset_str) {
        Ok(a) => a,
        Err(r) => {
            return Err(CliOutput(Err(crate::errors::TigerSharkError::CliError(
                format!("Bad Json format: {asset_str} : {r:?}"),
            ))));
        }
    };
    // to check if json values are present for the current command
    let a_name = asset_option.name.is_some();
    let a_location = asset_option.location.is_some();
    let a_asset_id = asset_option.asset_id.is_some();
    let a_version = asset_option.version.is_some();
    let a_version_id = asset_option.version_id.is_some();
    let a_datapath = asset_option.datapath.is_some();
    let a_source = asset_option.source.is_some();

    // unpack JsonOption into JsonString
    let asset: AssetJson = asset_option.into();

    fn keys_err(command: &str, asset: AssetJson) -> CliOutput {
        CliOutput(Err(crate::errors::TigerSharkError::AssetKeysError(
            command.to_owned(),
            format!("{:?}", asset),
        )))
    }
    // >>> COMMAND <<<
    // for each command, checks that the correct json values are present
    match args.command {
        CommandType::Insert => {
            match (a_name && a_location && a_datapath && a_source)
                || (a_asset_id && a_datapath && a_source)
            {
                true => Ok(Command {
                    command: CommandType::Insert,
                    json: Some(asset),
                    extra_args: None,
                }),
                _ => Err(keys_err("insert", asset)),
            }
        }
        CommandType::Source => {
            match (a_name && a_location || a_asset_id) && a_version || a_version_id {
                true => Ok(Command {
                    command: CommandType::Source,
                    json: Some(asset),
                    extra_args: None,
                }),
                _ => Err(keys_err("source", asset)),
            }
        }
        CommandType::Delete => {
            match (a_name && a_location || a_asset_id) && a_version || a_version_id {
                true => Ok(Command {
                    command: CommandType::Delete,
                    json: Some(asset),
                    extra_args: None,
                }),
                _ => Err(keys_err("delete", asset)),
            }
        }
        CommandType::Latest => match a_name && a_location || a_asset_id {
            true => Ok(Command {
                command: CommandType::Latest,
                json: Some(asset),
                extra_args: None,
            }),
            _ => Err(keys_err("latest", asset)),
        },
        CommandType::Approve => {
            match (a_name && a_location || a_asset_id) && a_version || a_version_id {
                true => Ok(Command {
                    command: CommandType::Approve,
                    json: Some(asset),
                    extra_args: None,
                }),
                _ => Err(keys_err("approve", asset)),
            }
        }
        // returns a list of versions to delete on disk
        CommandType::Purge => Ok(Command {
            command: CommandType::Purge,
            json: None,
            extra_args: None,
        }),
    }
}
