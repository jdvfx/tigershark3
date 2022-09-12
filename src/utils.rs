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

    // this part needs some cleanup
    // should use fetch_one() instead of fetch_all()
    //
    // if asset_id is missing, use name+location to get it instead
    let mut asset_id:i64 = json.asset_id;
    //
    if asset_id == 0_i64{
        let sql = sqlx::query(&format!(
            "
                SELECT asset_id FROM assets WHERE name='{na}' AND location='{lo}';
            ",
            na = json.name,
            lo = json.location,
        ))
        .fetch_all(&mut connection)
        .await;

        match sql{
            Ok(s) => {
                for i in s.iter(){
                    let x: Asset = i.into();
                    asset_id = x.asset_id;
                }
            },
            Err(_) => {
                return CliOutput::new("err", "asset ID not found, from name,location ");
            }
        }
    }

    let sql = sqlx::query(&format!(
        "
            SELECT version FROM versions WHERE asset_id='{}';
        ",
        &asset_id,
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

            let q = format!(
                "
                    INSERT INTO versions
                    ('asset_id','version','source','datapath','depend','approved','status')
                    VALUES ('{as}','{ve}','{so}','{da}','{de}','{ap}','{st}');
                ",
                as = &asset_id,
                ve = last_version + 1_i64,
                so = json.source,
                da = json.datapath,
                de = "",
                ap = 0,
                st = 0,
            );

            let sql = sqlx::query(&q)
            .execute(&mut connection)
            .await;
            match sql {
                Ok(_) => CliOutput::new("ok", "Asset Version Created"),
                Err(e) => CliOutput::new("err", &format!("Error creating Asset Version : {:?} {}", e, q)),
            }
        }
        Err(e) => CliOutput::new("err", "___err___"),
    }

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
