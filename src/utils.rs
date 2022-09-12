#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use crate::assetdef::Version;
use crate::errors::CliOutput;
use crate::parse_args::{Asset, AssetJson};
use sqlx::SqliteConnection;

pub async fn create(mut connection: SqliteConnection, json: AssetJson) -> CliOutput {
    let sql = sqlx::query(&format!(
        "
            INSERT INTO assets
            ('name','location') VALUES ('{}','{}');
        ",
        json.name, json.location
    ))
    .execute(&mut connection)
    .await;
    match sql {
        Ok(_) => CliOutput::new("ok", "Asset Created"),
        Err(e) => CliOutput::new("err", &format!("Error creating Asset : {:?}", e)),
    }
}

pub async fn update(mut connection: SqliteConnection, json: AssetJson) -> CliOutput {
    // version
    // source
    // datapath
    // depend
    // approved
    // status
    // asset_id .
    //

    let sql = sqlx::query(&format!(
        "
            SELECT version FROM versions WHERE asset_id='{}';
        ",
        json.asset_id,
    ))
    .fetch_all(&mut connection)
    .await;

    match sql {
        Ok(sql) => {
            let last_version = sql
                .iter()
                .map(|r| r.into())
                .collect::<Vec<Version>>()
                .iter()
                .map(|r| r.version)
                .max()
                .unwrap_or(0);
            println!(">>> last version <<< {:?}", last_version);

            let sql = sqlx::query(&format!(
                "
                    INSERT INTO versions
                    ('asset_id','version','source','datapath','depend','approved','status')
                    VALUES ('{as}','{ve}','{so}','{da}','{de}','{ap}','{st}');
                ",
                as = json.asset_id,
                ve = last_version + 1_i64,
                so = json.source,
                da = json.datapath,
                de = "",
                ap = 0,
                st = 0,
            ))
            .execute(&mut connection)
            .await;
            match sql {
                Ok(_) => CliOutput::new("ok", "Asset Version Created"),
                Err(e) => CliOutput::new("err", &format!("Error creating Asset Version : {:?}", e)),
            }
        }
        Err(e) => CliOutput::new("err", "___err___"),
    }

    //

    // pub asset_id: i64,
    // pub version_id: i64,
    // pub version: i64,
    // pub source: String,
    // pub datapath: String,
    // pub depend: String,
    // pub approved: u8,
    // pub status: u8,

    // name location asset_id: box
    // version - query first, then increment
    // source
    // datapath

    // depend (option)

    // approved: 0
    // status: 0
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
                "name"	    TEXT,
                "location"  TEXT
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
