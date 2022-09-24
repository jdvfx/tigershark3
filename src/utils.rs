#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]
//
use crate::assetdef::Version;
use crate::errors::CliOutput;
use crate::parse_args::{Asset, AssetJson};
use chrono::prelude::*;
use sqlx::Acquire;
use sqlx::Pool;
use sqlx::Sqlite;
// use sqlx::PoolConnection;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqlitePoolOptions;
//
fn now() -> String {
    let local: DateTime<Local> = Local::now();
    let d = local.date();
    let t = local.time();
    let dt = d.and_time(t);
    let now = dt.unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
    now
}

pub async fn insert(mut connection: PoolConnection<Sqlite>, mut json: AssetJson) -> CliOutput {
    // first, let's find out if the asset exists
    if json.asset_id != 0 {
        // asset exist, let's up-version then
        if !(json.source.is_empty() || json.datapath.is_empty()) {
            update(connection, json).await;
        }
    } else {
        // asset_id doesn't exist, use name+location
        let q = format!(
            "
                SELECT * FROM assets
                WHERE name='{na}' AND location='{lo}';
            ",
            na = json.name,
            lo = json.location,
        );

        let sql = sqlx::query(&q).fetch_one(&mut connection).await;
        if sql.is_err() {
            // asset ID not found, create a new asset
            let sql2 = sqlx::query(&format!(
                "
                    INSERT INTO assets
                    ('name','location') VALUES ('{}','{}');
                ",
                json.name, json.location
            ))
            .execute(&mut connection)
            .await;
            match sql2 {
                Ok(_sql2) => {
                    // new asset created
                    // let's find out its asset_id
                    let sql3 = sqlx::query(&format!(
                        "
                            SELECT * FROM assets
                            WHERE name='{na}' AND location='{lo}'; 
                        ",
                        na = json.name,
                        lo = json.location,
                    ))
                    .fetch_one(&mut connection)
                    .await;
                    match sql3 {
                        Ok(s) => {
                            // found the asset_id of the newly created asset
                            let asset: Asset = s.into();
                            json.asset_id = asset.asset_id;
                            if !(json.source.is_empty() || json.datapath.is_empty()) {
                                update(connection, json).await;
                            }
                        }
                        Err(e) => {
                            return CliOutput::new("err", &format!("Couldn't find ID: {:?}", e))
                        }
                    }
                }
                Err(e) => {
                    return CliOutput::new("err", &format!("Error creating Asset : {:?}", e));
                }
            }
        } else {
            let asset: Asset = sql.unwrap().into();
            json.asset_id = asset.asset_id;

            if !(json.source.is_empty() || json.datapath.is_empty()) {
                update(connection, json).await;
            }
        }
    }
    CliOutput::new("ok", "Asset Created")
}
pub async fn update(mut connection: PoolConnection<Sqlite>, json: AssetJson) -> CliOutput {
    // get last version
    let last_version: i64 = latest_version(&mut connection, json.asset_id).await;

    // add access date - last time the file got read (that can be updated every few days?)
    // don't want to update access date every single time it's accessed - too much for DB

    let q = format!(
        "
            INSERT INTO versions
            ('asset_id','version','source','datapath','depend','approved','status','ctime','atime')
            VALUES ('{as}','{ve}','{so}','{da}','{de}','{ap}','{st}','{ct}','{ct}');
        ",
        as = json.asset_id,
        ve = last_version + 1_i64,
        so = json.source,
        da = json.datapath,
        de = json.depend,
        ap = 0,
        st = 1,
        ct = now(),
    );

    let sql = sqlx::query(&q).execute(&mut connection).await;
    match sql {
        Ok(_) => CliOutput::new("ok", "Asset Version Created"),
        Err(e) => CliOutput::new(
            "err",
            &format!("Error creating Asset Version : {:?} {}", e, q),
        ),
    }
}

pub async fn latest_version(connection: &mut PoolConnection<Sqlite>, asset_id: i64) -> i64 {
    let sql = sqlx::query(&format!(
        "
            SELECT version FROM versions WHERE asset_id='{}';
        ",
        &asset_id,
    ))
    .fetch_all(connection)
    .await;

    let asset_id: i64 = match sql {
        Ok(sql) => {
            let last_version = sql
                .iter()
                .map(|r| r.into())
                .collect::<Vec<Version>>()
                .iter()
                .map(|r| r.version)
                .max()
                .unwrap_or(0);
            last_version
        }

        Err(_) => 0_i64,
    };
    asset_id
}

pub async fn source(mut connection: PoolConnection<Sqlite>, json: AssetJson) -> CliOutput {
    // get asset_id :  if json.asset.id is missing, use name and location to quiery it
    let asset_id_ = get_asset_id(&mut connection, json.clone()).await;
    let asset_id: i64 = match asset_id_ {
        Ok(a) => a,
        Err(cli) => return cli,
    };
    let q = format!(
        "
            SELECT source FROM versions WHERE asset_id='{as}' AND version='{ve}';
        ",
        as = &asset_id,
        ve = json.version,
    );

    let sql = sqlx::query(&q).fetch_one(&mut connection).await;

    match sql {
        Ok(s) => {
            let version: Version = s.into();
            let source = version.source;
            CliOutput::new("ok", &format!("source : {}", source))
        }
        Err(e) => CliOutput::new("err", &format!("Source not found: {:?} {}", e, q)),
    }
}

pub async fn delete(mut connection: PoolConnection<Sqlite>, json: AssetJson) -> CliOutput {
    let asset_id_ = get_asset_id(&mut connection, json.clone()).await;
    let asset_id: i64 = match asset_id_ {
        Ok(a) => a,
        Err(cli) => return cli,
    };

    let q = format!(
        "
            UPDATE versions
            SET status = 2
            WHERE asset_id = {as} AND version = {ve};
        ",
        as = &asset_id,
        ve = json.version,
    );

    let sql = sqlx::query(&q).execute(&mut connection).await;

    match sql {
        Ok(_) => CliOutput::new("ok", "version marked for purge"),
        Err(e) => CliOutput::new(
            "ok",
            &format!("Error, could not mark asset for purge:{:?}", e),
        ),
    }
}

pub async fn latest(mut connection: PoolConnection<Sqlite>, json: AssetJson) -> CliOutput {
    // get asset_id :  if json.asset.id is missing, use name and location to quiery it
    let asset_id_ = get_asset_id(&mut connection, json.clone()).await;
    let asset_id: i64 = match asset_id_ {
        Ok(a) => a,
        Err(cli) => return cli,
    };
    // get last version
    let last_version: i64 = latest_version(&mut connection, asset_id).await;
    CliOutput::new("ok", &format!("latest : {:?}", last_version))
}

pub async fn approve(connection: PoolConnection<Sqlite>, json: AssetJson) -> CliOutput {
    // get the version_id

    let version_id = get_version_id(&mut connection, json).await;
    println!("version id : {:?}", &version_id);

    let asset_id_ = get_asset_id(&mut connection, json.clone()).await;
    let asset_id: i64 = match asset_id_ {
        Ok(a) => a,
        Err(cli) => return cli,
    };
    // approve the current version
    let q = format!(
        "
            UPDATE versions
            SET approved = 1
            WHERE asset_id = {as} AND version = {ve};
        ",
        as = &asset_id,
        ve = json.version,
    );

    let sql = sqlx::query(&q).execute(&mut connection).await;
    match sql {
        Ok(_) => CliOutput::new("ok", "version Approved"),
        Err(e) => CliOutput::new("ok", &format!("Error, could not approve version:{:?}", e)),
    }

    // TO DO : approve dependencies as separate function
    // async fn approve_dependencies(connection: &mut PoolConnection<Sqlite>, depend: Vec<String>) {}
    //
    // approve dependencies
    let q = format!(
        "
            SELECT depend FROM versions WHERE asset_id='{as}' AND version='{ve}';
        ",
        as = &asset_id,
        ve = json.version,
    );

    let sql = sqlx::query(&q).fetch_one(&mut connection).await;
    let mut depend = "".to_string();
    if sql.is_ok() {
        let version: Version = sql.unwrap().into();
        println!(". . . version : {:?}", &version);
        depend = version.depend;
    }
    println!(">>> DEPEND >>> {:?}", depend);

    let version_id_depends: Vec<&str> = depend.split(",").collect();

    // TODO : remove this nasty unwrap
    // oh shit! unwraps everywhere...
    //
    let conn = connection.acquire().await.unwrap();
    let mut tx = conn.begin().await.unwrap();

    for version_id in version_id_depends {
        println!("--- version_id {}", version_id);
        sqlx::query(&format!(
            "
                UPDATE versions
                SET approved = 1
                WHERE version_id = {};
            ",
            version_id
        ))
        .execute(&mut tx)
        .await
        .unwrap();
    }

    tx.commit().await.unwrap();

    CliOutput::new("ok", "ALL GOOD")
}
// internal - get asset_id if name and location as not provided
async fn get_asset_id(
    connection: &mut PoolConnection<Sqlite>,
    json: AssetJson,
) -> Result<i64, CliOutput> {
    let asset_id: i64 = json.asset_id;
    //
    if asset_id == 0_i64 {
        let sql = sqlx::query(&format!(
            "
                SELECT asset_id FROM assets WHERE name='{na}' AND location='{lo}';
            ",
            na = json.name,
            lo = json.location,
        ))
        .fetch_one(connection)
        .await;

        match sql {
            Ok(s) => {
                let asset: Asset = s.into();
                return Ok(asset.asset_id);
            }
            Err(_) => {
                return Err(CliOutput::new(
                    "err",
                    "asset ID not found, from name,location ",
                ))
            }
        }
    }
    Ok(asset_id)
}

// >>> GET VERSION ID
pub async fn test(mut connection: PoolConnection<Sqlite>, json: AssetJson) -> CliOutput {
    // get the version_id
    let q = &format!(

        // this is broken, the version_id is WRONNNNNNGGGG!! 100% failure guaranteed
        "
            SELECT versions.*
            FROM assets, versions
            ON assets.asset_id = versions.asset_id
            WHERE name='{na}' AND location='{lo}' AND version='{ve}'
            OR versions.asset_id='{as}' AND version='{ve}';
        ",
        na = json.name,
        lo = json.location,
        ve = json.version,
        as = json.asset_id,
    );

    println!("q-{:}", q);

    // let sql = sqlx::query(&q).execute(&mut connection).await;
    let sql = sqlx::query(q).fetch_one(&mut connection).await;
    match sql {
        Ok(s) => {
            let version: Version = s.into();
            println!("{:?}", version);
            let version_id = version.version_id;
            println!(">>> {:?}", version_id);

            CliOutput::new("ok", "ok")
            // let version: Version = s.into();
            // return Ok(version.version_id);
        }
        Err(_) => return CliOutput::new("err", "version ID not found, from name,location "),
    }

    //
    // let asset_id: i64 = json.asset_id;
    // let version: i64 = json.version;
    //
    // println!("asset_id {:?} version {:?}", &asset_id, &version);
    //
    // let mut q: String = "".to_string();
    // if asset_id == 0_i64 {
    //     q = format!("
    //                 SELECT version_id FROM versions WHERE name='{na}' AND location='{lo}' AND version='{ve}';
    //             ",
    //             na=json.name,
    //             lo=json.location,
    //             ve=version
    //             );
    //     println!("name {:?} location {:?}", &json.name, &json.location);
    // } else {
    //     q = format!(
    //         "
    //                 SELECT version_id FROM versions WHERE asset_id='{as}' AND version='{ve}';
    //             ",
    //         as = asset_id,
    //         ve=version
    //     );
    // }
    //
    // println!(".. .. ");
    // println!("{:?}", &q);
    //
    // let sql = sqlx::query(&q).fetch_one(connection).await;
    // match sql {
    //     Ok(s) => {
    //         let version: Version = s.into();
    //         return Ok(version.version_id);
    //     }
    //     Err(_) => {
    //         return Err(CliOutput::new(
    //             "err",
    //             "version ID not found, from name,location ",
    //         ))
    //     }
    // }
}

//////////////////////////////////////////////////////////////
// -- used for tables initialization only --
//////////////////////////////////////////////////////////////

pub async fn initialize(mut connection: PoolConnection<Sqlite>) -> CliOutput {
    //
    //
    // >>>> TO DO <<<<
    // create database if it doesn't exist
    //
    //
    // if !sqlx::Sqlite::database_exists(&db_name).await {
    // sqlx::Sqlite::create_database(&db_name).await;
    // }

    // let sql = sqlx::query(&format!(
    //     "
    //         CREATE DATABASE IF NOT EXISTS '{}';
    //         ",
    //     &db_name
    // ))
    // .execute()
    // .await;
    // match sql {
    //     Ok(..) => CliOutput::new("ok", "database initialized"),
    //     Err(e) => CliOutput::new("err", &format!("error initializing database {:?}", e)),
    // }
    //
    let sql = sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS "assets" (
                "asset_id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "name"	    TEXT,
                "location"  TEXT
            );
            CREATE TABLE IF NOT EXISTS "versions" (
                "version_id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "version"       INTEGER,
                "source"	TEXT,
                "datapath"	TEXT,
                "depend"	TEXT,
                "approved"	INTEGER,
                "status"	INTEGER,
                "creationtime"  TEXT,
                "asset_id"	INTEGER NOT NULL,
                FOREIGN KEY("asset_id") REFERENCES "assets"("asset_id")
            );
        "#,
    )
    .execute(&mut connection)
    .await;
    match sql {
        Ok(_) => CliOutput::new("ok", "'assets' and 'versions' tables created"),
        Err(e) => {
            return CliOutput::new(
                "ok",
                &format!("Error creating 'assets' and 'versions' tables :{:?}", e),
            )
        }
    }
}
