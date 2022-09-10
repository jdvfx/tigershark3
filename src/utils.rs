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

pub async fn create_asset_table(mut connection: SqliteConnection) -> CliOutput {
    let sql = sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS "assets" (
                "asset_id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "name"	TEXT
            );
        "#,
    )
    .execute(&mut connection)
    .await;
    match sql {
        Ok(_) => CliOutput::new("ok", "Asset table created"),
        Err(e) => CliOutput::new("ok", &format!("Error creating Asset table :{:?}", e)),
    }
}
pub async fn create_versions_table(mut connection: SqliteConnection) -> CliOutput {
    let sql = sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS "versions" (
                "version_id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "version"       INTEGER,
                "source"	TEXT,
                "datapath"	TEXT,
                "depend"	TEXT,
                "approved"	INTEGER,
                "status"	INTEGER,
                "asset_id"	INTEGER NOT NULL,
                FOREIGN KEY("asset_id") REFERENCES "assets"("asset_id")
            );
        "#,
    )
    .execute(&mut connection)
    .await;
    match sql {
        Ok(_) => CliOutput::new("ok", "Versions table created"),
        Err(e) => CliOutput::new("ok", &format!("Error creating Versions table :{:?}", e)),
    }
}
