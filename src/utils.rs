use crate::assetdef::Version;
use crate::errors::CliOutput;
use crate::parse_args::{Asset, AssetJson};
use chrono::prelude::*;
use sqlx::{pool::PoolConnection, Acquire, Sqlite};
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
            return create_version(connection, json).await;
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
                            // new asset created
                            if !(json.source.is_empty() || json.datapath.is_empty()) {
                                return create_version(connection, json).await;
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
                return create_version(connection, json).await;
            }
        }
    }
    CliOutput::new("ok", "Asset Created")
}

pub async fn create_version(mut connection: PoolConnection<Sqlite>, json: AssetJson) -> CliOutput {
    // get last version
    let last_version: i64 = match latest_version(&mut connection, json.asset_id).await {
        Ok(v) => v,
        Err(_) => 0_i64,
    };

    let new_version: i64 = last_version + 1_i64;
    // add access date - last time the file got read (that can be updated every few days?)
    // don't want to update access date every single time it's accessed - too much for DB

    let q = format!(
        "
            INSERT INTO versions
            ('asset_id','version','source','datapath','depend','approved','status','ctime','atime')
            VALUES ('{ass}','{ve}','{so}','{da}','{de}','{ap}','{st}','{ct}','{ct}');
        ",
        ass = json.asset_id,
        ve = new_version,
        so = json.source,
        da = json.datapath,
        de = json.depend,
        ap = 0,
        st = 1,
        ct = now(),
    );

    // should be able to add RETURNING asset_id   after VALUES ()
    //

    let sql = sqlx::query(&q).execute(&mut connection).await;
    match sql {
        Ok(_) => return CliOutput::new("ok", &format!("{new_version}")),
        Err(e) => {
            return CliOutput::new(
                "err",
                &format!("Error creating Asset Version : {:?} {}", e, q),
            )
        }
    }
}

pub async fn latest_version(
    connection: &mut PoolConnection<Sqlite>,
    asset_id: i64,
) -> Result<i64, String> {
    let sql = sqlx::query(&format!(
        "
            SELECT version FROM versions WHERE asset_id='{}';
        ",
        &asset_id,
    ))
    .fetch_all(connection)
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
            Ok(last_version)
        }
        Err(e) => Err(format!("Error:{:?}", e)),
    }
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
            SELECT source FROM versions WHERE asset_id='{ass}' AND version='{ve}';
        ",
        ass = &asset_id,
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
            WHERE asset_id = {ass} AND version = {ve};
        ",
        ass = &asset_id,
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
    match latest_version(&mut connection, asset_id).await {
        Ok(v) => CliOutput::new("ok", &format!("latest : {:?}", v)),
        Err(e) => CliOutput::new("err", &format!("no version found: {:?}", e)),
    }
}

pub async fn approve(mut connection: PoolConnection<Sqlite>, json: AssetJson) -> CliOutput {
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
            WHERE asset_id = {ass} AND version = {ve};
        ",
        ass = &asset_id,
        ve = json.version,
    );

    let sql = sqlx::query(&q).execute(&mut connection).await;
    match sql {
        Ok(_) => (),
        Err(e) => {
            return CliOutput::new("err", &format!("Error, could not approve version: {:?}", e))
        }
    }

    // approve dependencies
    let q = format!(
        "
            SELECT depend FROM versions WHERE asset_id='{ass}' AND version='{ve}';
        ",
        ass = &asset_id,
        ve = json.version,
    );

    let sql = sqlx::query(&q).fetch_one(&mut connection).await;
    let mut depend = "".to_string();
    if sql.is_ok() {
        let version: Version = sql.unwrap().into();
        depend = version.depend;
    }

    let version_id_depends: Vec<&str> = depend.split(",").collect();
    let d = approve_dependencies(connection, version_id_depends).await;
    match d {
        Ok(_) => CliOutput::new("ok", "Asset and Dependencies approved"),
        Err(e) => CliOutput::new("err", &format!("Error approving dependencies: {:?}", e)),
    }
}

// internal - approve all versions in version.depend
async fn approve_dependencies(
    mut connection: PoolConnection<Sqlite>,
    version_id_depends: Vec<&str>,
) -> Result<(), sqlx::Error> {
    let conn = connection.acquire().await?;
    let mut tx = conn.begin().await?;

    for version_id in version_id_depends {
        sqlx::query(&format!(
            "
                UPDATE versions
                SET approved = 1
                WHERE version_id = {};
            ",
            version_id
        ))
        .execute(&mut tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
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

// initialize assets and versions tables
pub async fn initialize(mut connection: PoolConnection<Sqlite>) -> CliOutput {
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
                "ctime"         TEXT,
                "atime"         TEXT,
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
