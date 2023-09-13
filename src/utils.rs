use crate::assetdef::{Status, Version};
use crate::errors::{CliOutput, TigerSharkError};
use crate::frame_num::find_replace_frame_num;
use crate::parse_args::{Asset, AssetJson};

use chrono::prelude::*;
use sqlx::{pool::PoolConnection, Acquire, Sqlite};

async fn create_asset(
    connection: &mut PoolConnection<Sqlite>,
    json: &AssetJson,
) -> Result<String, String> {
    let sql = sqlx::query("INSERT INTO assets ('name','location') VALUES (?,?);")
        .bind(&json.name)
        .bind(&json.location)
        .execute(connection)
        .await;
    match sql {
        Err(e) => Err(e.to_string()),
        Ok(_) => Ok("".to_string()),
    }
}
//
pub async fn find_asset_id(
    connection: &mut PoolConnection<Sqlite>,
    json: &AssetJson,
) -> Option<i64> {
    let sql = sqlx::query("SELECT * FROM assets WHERE name=? AND location=?;")
        .bind(&json.name)
        .bind(&json.location)
        .fetch_one(connection)
        .await;
    match sql {
        Ok(sql) => {
            // into() does unwrap_or_default() and returns asset_id=0 when fail.
            let asset: Asset = sql.into();
            if asset.asset_id == 0_i64 {
                return None;
            }
            Some(asset.asset_id)
        }
        Err(_) => None,
    }
}

pub async fn insert(mut connection: PoolConnection<Sqlite>, mut json: AssetJson) -> CliOutput {
    // doesn't have asset_id/not been passed in the assetJson
    if json.asset_id == 0 {
        let asset_id = find_asset_id(&mut connection, &json).await;
        if asset_id.is_none() {
            let c = create_asset(&mut connection, &json).await;
            if c.is_err() {
                return CliOutput(Err(TigerSharkError::DbError(format!(
                    "Error Creating Asset {:?}",
                    c
                ))));
            }
        }

        match find_asset_id(&mut connection, &json).await {
            None => {
                return CliOutput(Err(TigerSharkError::DbError(
                    "Error Finding Asset Version".to_string(),
                )))
            }
            Some(asset_id) => {
                json.asset_id = asset_id;
            }
        }
    }
    create_version(connection, json).await
}

pub async fn create_version(
    mut connection: PoolConnection<Sqlite>,
    mut json: AssetJson,
) -> CliOutput {
    // get last version
    let last_version: i64 = latest_version(&mut connection, json.asset_id)
        .await
        .unwrap_or(0_i64);
    // ignore error and create v1
    let new_version: i64 = last_version + 1_i64;

    // add access date - last time the file got read (that can be updated every few days?)
    // don't want to update access date every single time it's accessed - too much for DB

    // remove frame number and replace with ####
    let file = &json.datapath;
    let generic_frame = find_replace_frame_num(&file);
    if generic_frame.is_err() {
        return CliOutput(Err(TigerSharkError::FilePathError(
            "file argument parsing Error".to_string(),
        )));
    }
    let generic_frame = generic_frame.unwrap_or("".to_string());
    json.datapath = generic_frame;
    //
    //
    let q = "INSERT INTO versions
            ('asset_id','version','source','datapath','depend','approved','status','ctime','atime')
            VALUES (?,?,?,?,?,?,?,?,?);";
    let sql = sqlx::query(q)
        .bind(json.asset_id)
        .bind(new_version)
        .bind(json.source)
        .bind(json.datapath)
        .bind(json.depend)
        .bind(0)
        .bind(Status::Online as u8)
        .bind(now())
        .bind(now())
        .execute(&mut connection)
        .await;

    // let sql = sqlx::query(q).execute(&mut connection).await;
    match sql {
        Ok(s) => {
            // return row_id which is version_id in this case
            let rowid = s.last_insert_rowid();
            // CliOutput(Ok(format!("{rowid:?}")))
            CliOutput(Ok(rowid.to_string()))
        }
        Err(e) => CliOutput(Err(TigerSharkError::DbError(format!(
            "Error creating Asset Version : {e:?} {q}"
        )))),
    }
}

pub async fn latest_version(
    connection: &mut PoolConnection<Sqlite>,
    asset_id: i64,
) -> Result<i64, String> {
    let sql = sqlx::query("SELECT version FROM versions WHERE asset_id=?;")
        .bind(asset_id)
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
        Err(e) => Err(format!("Error:{e:?}")),
    }
}

pub async fn source(mut connection: PoolConnection<Sqlite>, mut json: AssetJson) -> CliOutput {
    //
    find_asset_id_and_version(&mut connection, &mut json).await;
    if json.asset_id == 0_i64 || json.version == 0_i64 {
        return CliOutput(Err(TigerSharkError::NotFound(
            "could not find asset_id and version".to_owned(),
        )));
    }

    let q = "SELECT source FROM versions WHERE asset_id=? AND version=?;";
    let sql = sqlx::query(q)
        .bind(json.asset_id)
        .bind(json.version)
        .fetch_one(&mut connection)
        .await;

    match sql {
        Ok(s) => {
            let version: &Version = &(&s).into();
            let source = &version.source;
            CliOutput(Ok(format!("source : {source}")))
        }
        Err(e) => CliOutput(Err(TigerSharkError::NotFound(format!(
            "Source not found: {e:?} {q}"
        )))),
    }
}

pub async fn source_from_file(mut connection: PoolConnection<Sqlite>, file: String) -> CliOutput {
    let generic_frame = find_replace_frame_num(&file);
    if generic_frame.is_err() {
        return CliOutput(Err(TigerSharkError::FilePathError(
            "file argument parsing Error".to_string(),
        )));
    }
    let generic_frame = generic_frame.unwrap_or("".to_string());

    let q = "SELECT source FROM versions WHERE datapath=? ;";
    let sql = sqlx::query(q)
        .bind(generic_frame)
        .fetch_one(&mut connection)
        .await;

    match sql {
        Ok(s) => {
            let version: &Version = &(&s).into();
            let source = &version.source;
            CliOutput(Ok(format!("{source}")))
        }
        Err(e) => CliOutput(Err(TigerSharkError::NotFound(format!(
            "Source not found: {e:?} {q}"
        )))),
    }
}

pub async fn delete(mut connection: PoolConnection<Sqlite>, mut json: AssetJson) -> CliOutput {
    //
    find_asset_id_and_version(&mut connection, &mut json).await;
    if json.asset_id == 0_i64 || json.version == 0_i64 {
        return CliOutput(Err(TigerSharkError::NotFound(
            "could not find asset_id and version".to_owned(),
        )));
    }

    let q = "UPDATE versions
             SET status = ?
             WHERE asset_id = ? AND version = ?;";
    let sql = sqlx::query(q)
        .bind(Status::Purge as u8)
        .bind(json.asset_id)
        .bind(json.version)
        .execute(&mut connection)
        .await;

    match sql {
        Ok(_) => CliOutput(Ok("version marked for purge".to_owned())),
        Err(e) => CliOutput(Err(TigerSharkError::DbError(format!(
            "Error, could not mark asset for purge:{e:?}"
        )))),
    }
}

pub async fn latest(mut connection: PoolConnection<Sqlite>, mut json: AssetJson) -> CliOutput {
    // get asset_id :  if json.asset.id is missing, use name+location or version_id to quiery it
    find_asset_id_and_version(&mut connection, &mut json).await;
    if json.asset_id == 0_i64 {
        return CliOutput(Err(TigerSharkError::NotFound(
            "Error, could not find asset_id".to_owned(),
        )));
    }
    // get last version
    match latest_version(&mut connection, json.asset_id).await {
        Ok(v) => CliOutput(Ok(format!("{v:?}"))),
        Err(e) => CliOutput(Err(TigerSharkError::NotFound(format!(
            "no version found: {e:?}"
        )))),
    }
}

pub async fn approve(mut connection: PoolConnection<Sqlite>, mut json: AssetJson) -> CliOutput {
    //
    find_asset_id_and_version(&mut connection, &mut json).await;
    if json.asset_id == 0_i64 || json.version == 0_i64 {
        return CliOutput(Err(TigerSharkError::NotFound(
            "Error, could not find asset_id and version".to_owned(),
        )));
    }

    let q = "UPDATE versions
            SET approved = 1
            WHERE asset_id = ? AND version = ?;";
    let sql = sqlx::query(q)
        .bind(json.asset_id)
        .bind(json.version)
        .execute(&mut connection)
        .await;
    match sql {
        Ok(_) => (),
        Err(e) => {
            return CliOutput(Err(TigerSharkError::DbError(format!(
                "Error, could not approve version: {e:?}"
            ))))
        }
    }

    // approve dependencies
    let q = "SELECT depend FROM versions WHERE asset_id=? AND version=?;";
    let sql = sqlx::query(q)
        .bind(json.asset_id)
        .bind(json.version)
        .fetch_one(&mut connection)
        .await;

    let mut depend = "".to_owned();
    if sql.is_ok() {
        let version: &Version = &(&sql.unwrap()).into();
        depend = version.clone().depend;
    }

    let version_id_depends: Vec<&str> = depend.split(',').filter(|x| !x.is_empty()).collect();

    match version_id_depends.len() {
        0 => CliOutput(Ok("Asset approved, no dependencies".to_owned())),
        _ => {
            let d = approve_dependencies(connection, version_id_depends).await;
            match d {
                Ok(_) => CliOutput(Ok("Asset and Dependencies approved".to_owned())),
                Err(e) => CliOutput(Err(TigerSharkError::DbError(format!(
                    "Error approving dependencies: {e:?}"
                )))),
            }
        }
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
        sqlx::query(
            "UPDATE versions
             SET approved = 1
             WHERE version_id = ?;",
        )
        .bind(version_id)
        .execute(&mut tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

async fn find_asset_id_and_version(connection: &mut PoolConnection<Sqlite>, json: &mut AssetJson) {
    let asset_id: i64 = json.asset_id;
    let version_id: i64 = json.version_id;
    // if no asset id found, get it from name+location or version_id
    if asset_id == 0_i64 {
        if version_id == 0_i64 {
            let q = "SELECT asset_id FROM assets WHERE name=? AND location=?;";
            let sql = sqlx::query(q)
                .bind(&json.name)
                .bind(&json.location)
                .fetch_one(connection)
                .await;
            if sql.is_ok() {
                let asset: Asset = sql.unwrap().into();
                json.asset_id = asset.asset_id;
            }
        } else {
            let q = "SELECT asset_id,version FROM versions WHERE version_id=?;";
            let sql = sqlx::query(q).bind(version_id).fetch_one(connection).await;
            if sql.is_ok() {
                let version: &Version = &(&sql.unwrap()).into();
                json.asset_id = version.asset_id;
                // version might be provided by the user, or not, just get it.
                json.version = version.version;
            }
        }
    }
}

pub async fn purge(mut connection: PoolConnection<Sqlite>) -> CliOutput {
    // find asset for purge
    let q = "SELECT * FROM versions
             WHERE status=?;";
    let sql = sqlx::query(q)
        .bind(Status::Purge as u8)
        .fetch_all(&mut connection)
        .await;

    match sql {
        Ok(sql) => {
            let versions_to_purge: Vec<String> = sql
                .iter()
                .map(|r| r.into())
                .collect::<Vec<Version>>()
                .iter()
                .map(|r| r.datapath.clone())
                .collect::<Vec<String>>();

            let mut s: String = String::from("");
            for i in versions_to_purge.iter() {
                s.push_str(i);
                s.push('#');
            }
            CliOutput(Ok(format!("{s:?}")))
        }
        Err(e) => CliOutput(Err(TigerSharkError::NotFound(format!(
            "Cannot access versions to purge {e:?}"
        )))),
    }
}

fn now() -> String {
    let local: DateTime<Local> = Local::now();
    let date = local.date_naive();
    let time = local.time();
    let datetime = date.and_time(time);
    let now = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    now
}
