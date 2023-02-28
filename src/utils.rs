use crate::assetdef::{Status, Version};
use crate::errors::{CliOutput, TigerSharkError};
use crate::parse_args::{Asset, AssetJson};
use chrono::prelude::*;
use sqlx::{pool::PoolConnection, Acquire, Sqlite};
//

// first, let's find out if the asset exists
pub async fn insert(mut connection: PoolConnection<Sqlite>, mut json: AssetJson) -> CliOutput {
    // first, let's find out if the asset exists
    if json.asset_id == 0 {
        create_version(connection, json).await
    } else {
        // asset_id doesn't exist, use name+location
        let q = format!(
            "   SELECT * FROM assets
                WHERE name='{na}' AND location='{lo}';",
            na = json.name,
            lo = json.location,
        );

        let sql = sqlx::query(&q).fetch_one(&mut connection).await;
        if sql.is_err() {
            // asset ID not found, create a new asset
            let sql2 = sqlx::query(&format!(
                "   INSERT INTO assets
                    ('name','location') VALUES ('{}','{}');",
                json.name, json.location
            ))
            .execute(&mut connection)
            .await;
            match sql2 {
                Ok(_sql2) => {
                    // new asset created
                    // let's find out its asset_id
                    let sql3 = sqlx::query(&format!(
                        "   SELECT * FROM assets
                            WHERE name='{na}' AND location='{lo}';",
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
                            create_version(connection, json).await
                        }
                        Err(e) => CliOutput(Err(TigerSharkError::NotFound(format!(
                            "Couldn't find ID: {e:?}"
                        )))),
                    }
                }
                Err(e) => CliOutput(Err(TigerSharkError::DbError(format!(
                    "Error creating Asset : {e:?}"
                )))),
            }
        } else {
            let asset: Asset = sql.unwrap().into();
            json.asset_id = asset.asset_id;
            create_version(connection, json).await
        }
    }
}

pub async fn create_version(mut connection: PoolConnection<Sqlite>, json: AssetJson) -> CliOutput {
    // get last version
    let last_version: i64 = latest_version(&mut connection, json.asset_id)
        .await
        .unwrap_or(0_i64);

    // print!("?? {}", &last_version);

    let new_version: i64 = last_version + 1_i64;
    // add access date - last time the file got read (that can be updated every few days?)
    // don't want to update access date every single time it's accessed - too much for DB

    let q = format!(
        "   INSERT INTO versions
            ('asset_id','version','source','datapath','depend','approved','status','ctime','atime')
            VALUES ('{ass}','{ve}','{so}','{da}','{de}','{ap}','{st}','{ct}','{ct}');",
        ass = json.asset_id,
        ve = new_version,
        so = json.source,
        da = json.datapath,
        de = json.depend,
        ap = 0,
        st = Status::Online as u8,
        ct = now(),
    );

    let sql = sqlx::query(&q).execute(&mut connection).await;
    match sql {
        Ok(s) => {
            // return row_id which is version_id in this case
            let rowid = s.last_insert_rowid();
            CliOutput(Ok(format!("{rowid:?}")))
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
    let sql = sqlx::query(&format!(
        "SELECT version FROM versions WHERE asset_id='{}';",
        asset_id,
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
        Err(e) => Err(format!("Error:{e:?}")),
    }
}

pub async fn source(mut connection: PoolConnection<Sqlite>, mut json: AssetJson) -> CliOutput {
    //
    find_asset_id_and_version(&mut connection, &mut json).await;
    if json.asset_id == 0_i64 || json.version == 0_i64 {
        return CliOutput(Err(TigerSharkError::NotFound(
            "could not find asset_id and version".to_string(),
        )));
    }

    let q = format!(
        "   SELECT source FROM versions WHERE asset_id='{ass}' AND version='{ve}';",
        ass = json.asset_id,
        ve = json.version,
    );

    let sql = sqlx::query(&q).fetch_one(&mut connection).await;

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

pub async fn delete(mut connection: PoolConnection<Sqlite>, mut json: AssetJson) -> CliOutput {
    //
    find_asset_id_and_version(&mut connection, &mut json).await;
    if json.asset_id == 0_i64 || json.version == 0_i64 {
        return CliOutput(Err(TigerSharkError::NotFound(
            "could not find asset_id and version".to_string(),
        )));
    }

    let q = format!(
        "   UPDATE versions
            SET status = {st}
            WHERE asset_id = {ass} AND version = {ve};",
        st = Status::Purge as u8,
        ass = json.asset_id,
        ve = json.version,
    );

    let sql = sqlx::query(&q).execute(&mut connection).await;

    match sql {
        Ok(_) => CliOutput(Ok("version marked for purge".to_string())),
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
            "Error, could not find asset_id".to_string(),
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
            "Error, could not find asset_id and version".to_string(),
        )));
    }

    let q = format!(
        "   UPDATE versions
            SET approved = 1
            WHERE asset_id = {ass} AND version = {ve};",
        ass = json.asset_id,
        ve = json.version,
    );

    let sql = sqlx::query(&q).execute(&mut connection).await;
    match sql {
        Ok(_) => (),
        Err(e) => {
            return CliOutput(Err(TigerSharkError::DbError(format!(
                "Error, could not approve version: {e:?}"
            ))))
        }
    }

    // approve dependencies
    let q = format!(
        "   SELECT depend FROM versions WHERE asset_id='{ass}' AND version='{ve}';",
        ass = json.asset_id,
        ve = json.version,
    );

    let sql = sqlx::query(&q).fetch_one(&mut connection).await;

    let mut depend = "".to_string();
    if sql.is_ok() {
        let version: &Version = &(&sql.unwrap()).into();
        depend = version.clone().depend;
    }

    let version_id_depends: Vec<&str> = depend.split(',').filter(|x| !x.is_empty()).collect();

    match version_id_depends.len() {
        0 => CliOutput(Ok("Asset approved, no dependencies".to_string())),
        _ => {
            let d = approve_dependencies(connection, version_id_depends).await;
            match d {
                Ok(_) => CliOutput(Ok("Asset and Dependencies approved".to_string())),
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

async fn find_asset_id_and_version(connection: &mut PoolConnection<Sqlite>, json: &mut AssetJson) {
    let asset_id: i64 = json.asset_id;
    let version_id: i64 = json.version_id;
    // if no asset id found, get it from name+location or version_id
    if asset_id == 0_i64 {
        if version_id == 0_i64 {
            let q = format!(
                "   SELECT asset_id FROM assets WHERE name='{na}' AND location='{lo}';",
                na = json.name,
                lo = json.location,
            );
            let sql = sqlx::query(&q).fetch_one(connection).await;
            if sql.is_ok() {
                let asset: Asset = sql.unwrap().into();
                json.asset_id = asset.asset_id;
            }
        } else {
            let q = format!(
                "   SELECT asset_id,version FROM versions WHERE version_id='{ve}';",
                ve = version_id
            );
            let sql = sqlx::query(&q).fetch_one(connection).await;
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
    let q = format!(
        "   SELECT * FROM versions
            WHERE status='{st}';",
        st = Status::Purge as u8
    );
    let sql = sqlx::query(&q).fetch_all(&mut connection).await;

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

// pub async fn test(connection: PoolConnection<Sqlite>) -> CliOutput {
//     CliOutput::new("ok", &format!("test good , {:?}", connection))
// }
//
fn now() -> String {
    let local: DateTime<Local> = Local::now();
    let date = local.date_naive();
    let time = local.time();
    let datetime = date.and_time(time);
    let now = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    now
}
