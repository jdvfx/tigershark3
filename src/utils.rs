use crate::assetdef::Version;
use crate::errors::CliOutput;
use crate::parse_args::{Asset, AssetJson};
use sqlx::SqliteConnection;

pub async fn create(mut connection: SqliteConnection, json: AssetJson) -> CliOutput {
    //
    let mut asset_id: i64 = 0;
    // first, let's find out if the asset exists
    if json.asset_id != 0 {
        // looks like the asset exist
        asset_id = json.asset_id;
    } else {
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
            println!("asset not in DB , create it");
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
                Ok(sql2) => {
                    println!("asset_created !!!");
                    println!("{:?}", sql2);

                    // super cool but doesn't return the asset_id //
                    //
                    // TO DO : find the asset_id !!!
                    //
                }
                Err(e) => {
                    return CliOutput::new("err", &format!("Error creating Asset : {:?}", e));
                }
            }
        } else {
            let asset: Asset = sql.unwrap().into();
            asset_id = asset.asset_id;
            println!("asset_id = {}", asset_id);
        }
    }

    println!("> > > asset ID: {}", asset_id);

    CliOutput::new("ok", "Asset Created")
}

// pub async fn create(mut connection: SqliteConnection, json: AssetJson) -> CliOutput {
//     let sql = sqlx::query(&format!(
//         "
//             INSERT INTO assets
//             ('name','location') VALUES ('{}','{}');
//         ",
//         json.name, json.location
//     ))
//     .execute(&mut connection)
//     .await;
//     match sql {
//         Ok(_) => CliOutput::new("ok", "Asset Created"),
//         Err(e) => CliOutput::new("err", &format!("Error creating Asset : {:?}", e)),
//     }
// }

pub async fn update(mut connection: SqliteConnection, json: AssetJson) -> CliOutput {
    //
    // get asset_id :  if json.asset.id is missing, use name and location to quiery it
    let asset_id_ = get_asset_id(&mut connection, json.clone()).await;
    let asset_id: i64 = match asset_id_ {
        Ok(a) => a,
        Err(cli) => return cli,
    };
    // get last version
    let last_version: i64 = latest_version(&mut connection, asset_id).await;

    // add creation date -  chrono::DateTime<Utc>
    // add access date - last time the file got read (that can be updated every few days?)
    // don't want to update access date every single time it's accessed - too much for DB

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
        st = 1,
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

pub async fn latest_version(connection: &mut SqliteConnection, asset_id: i64) -> i64 {
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

pub async fn source(mut connection: SqliteConnection, json: AssetJson) -> CliOutput {
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

    let sql = sqlx::query(&q).fetch_all(&mut connection).await;

    match sql {
        Ok(s) => {
            //
            let mut source: String = "_".to_string();
            for i in s.iter() {
                let x: Version = i.into();
                source = x.source;
            }

            CliOutput::new("ok", &format!("source : {}", source))
        }
        Err(e) => CliOutput::new("err", &format!("Source not found: {:?} {}", e, q)),
    }
}

pub async fn delete(mut connection: SqliteConnection, json: AssetJson) -> CliOutput {
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

    let sql = sqlx::query(&q).fetch_all(&mut connection).await;

    match sql {
        Ok(_) => CliOutput::new("ok", "version marked for purge"),
        Err(e) => CliOutput::new(
            "ok",
            &format!("Error, could not mark asset for purge:{:?}", e),
        ),
    }
}

pub async fn latest(mut connection: SqliteConnection, json: AssetJson) -> CliOutput {
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

pub async fn approve(mut connection: SqliteConnection, json: AssetJson) -> CliOutput {
    let asset_id_ = get_asset_id(&mut connection, json.clone()).await;
    let asset_id: i64 = match asset_id_ {
        Ok(a) => a,
        Err(cli) => return cli,
    };

    // TODO
    // * approve all the dependencies
    // * check the depend string
    // * split string, get at list of versions_id
    // * push queries into a container
    // * execute all

    let q = format!(
        "
            UPDATE versions
            SET approved = 1
            WHERE asset_id = {as} AND version = {ve};
        ",
        as = &asset_id,
        ve = json.version,
    );

    let sql = sqlx::query(&q).fetch_all(&mut connection).await;

    match sql {
        Ok(_) => CliOutput::new("ok", "version Approved"),
        Err(e) => CliOutput::new("ok", &format!("Error, could not approve version:{:?}", e)),
    }
}

// internal - get asset_id if name and location as not provided
async fn get_asset_id(
    connection: &mut SqliteConnection,
    json: AssetJson,
) -> Result<i64, CliOutput> {
    let mut asset_id: i64 = json.asset_id;
    //
    if asset_id == 0_i64 {
        let sql = sqlx::query(&format!(
            "
                SELECT asset_id FROM assets WHERE name='{na}' AND location='{lo}';
            ",
            na = json.name,
            lo = json.location,
        ))
        .fetch_all(connection)
        .await;

        match sql {
            Ok(s) => {
                for i in s.iter() {
                    let x: Asset = i.into();
                    asset_id = x.asset_id;
                }
                return Ok(asset_id);
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

//////////////////////////////////////////////////////////////
// -- used for tables initialization only --
//////////////////////////////////////////////////////////////

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
