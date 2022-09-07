#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use sqlx::sqlite;
use sqlx::Connection;

use crate::assetdef::Asset;
mod assetdef;
//
async fn create_assets_table(db_name: &str) -> Result<(), sqlx::Error> {
    let mut conn = sqlite::SqliteConnection::connect(db_name).await?;
    // TO DO : use the result of _ct_assets_
    let _ct_assets = sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS "assets" (
                "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "name"	TEXT
            );
        "#,
    )
    .execute(&mut conn)
    .await?;
    Ok(())
}

async fn create_versions_table(db_name: &str) -> Result<(), sqlx::Error> {
    let mut conn = sqlite::SqliteConnection::connect(&db_name).await?;
    // TO DO : use the result of _ct_assets_
    let _ct_assets = sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS "versions" (
                "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "source"	TEXT,
                "datapath"	TEXT,
                "depend"	TEXT,
                "approved"	INTEGER,
                "status"	INTEGER,
                "asset_id"	INTEGER NOT NULL,
                FOREIGN KEY("asset_id") REFERENCES "assets"("id")
            );
        "#,
    )
    .execute(&mut conn)
    .await?;
    Ok(())
}

async fn insert_asset(db_name: &str, asset_name: &str) -> Result<(), sqlx::Error> {
    let mut conn = sqlite::SqliteConnection::connect(&db_name).await?;
    // TO DO : use the result of _ct_assets_
    //
    //
    //
    let _ct_assets = sqlx::query(&format!(
        "
            INSERT INTO assets
            ('name') VALUES ('{}');
        ",
        asset_name
    ))
    .execute(&mut conn)
    .await?;
    Ok(())
}

async fn insert_version(db_name: &str) -> Result<(), sqlx::Error> {
    let mut conn = sqlite::SqliteConnection::connect(&db_name).await?;
    // TO DO : use the result of _ct_assets_
    let _ct_assets = sqlx::query(
        r#"
            INSERT INTO versions
            ("source","datapath","depend","approved","status","asset_id") 
            VALUES ("source_cone","datapath_cone","depend_cone",0,0,3);
        "#,
    )
    .execute(&mut conn)
    .await?;
    Ok(())
}

async fn find_asset_id(db_name: &str, asset_name: &str) -> Result<(), sqlx::Error> {
    let mut conn = sqlite::SqliteConnection::connect(&db_name).await?;
    // TO DO : use the result of _ct_assets_
    let _ct_assets = sqlx::query(&format!(
        "
            SELECT * FROM assets WHERE name='{}';
        ",
        asset_name
    ))
    .fetch_all(&mut conn)
    .await?;

    // could do something better with closure and Turbofish...
    for i in _ct_assets.iter() {
        let x: Asset = i.into();
        let id: i64 = x.id;
        println!("{:?}", id);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    //
    let db_name = "sqlite:/home/bunker/assets2.db";
    //
    // create_assets_table(&db_name).await?;
    // create_versions_table(&db_name).await?;
    // insert_version(&db_name).await?;
    // insert_asset(&db_name, "my_new_asset").await?;
    find_asset_id(&db_name, "cone").await?;
    //
    Ok(())
}
