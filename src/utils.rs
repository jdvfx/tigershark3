#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use crate::errors::CliOutput;
use crate::parse_args::{Asset, AssetJson};
use sqlx::SqliteConnection;

pub async fn create(connection: SqliteConnection, json: AssetJson) -> CliOutput {
    CliOutput::new("ok", "create")
}

pub async fn update(connection: SqliteConnection, json: AssetJson) -> CliOutput {
    CliOutput::new("ok", "update")
}

pub async fn source(connection: SqliteConnection, json: AssetJson) -> CliOutput {
    CliOutput::new("ok", "source")
}

pub async fn delete(connection: SqliteConnection, json: AssetJson) -> CliOutput {
    CliOutput::new("ok", "delete")
}

pub async fn latest(connection: SqliteConnection, json: AssetJson) -> CliOutput {
    CliOutput::new("ok", "latest")
}

pub async fn approve(connection: SqliteConnection, json: AssetJson) -> CliOutput {
    CliOutput::new("ok", "approve")
}
