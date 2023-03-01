use sqlx::sqlite::SqliteRow;
use sqlx::Row;

#[derive(sqlx::FromRow, Debug, Clone, Default)]
pub struct Asset {
    pub asset_id: i64,
    pub name: String,
    pub location: String,
}

// Sqlite results into Asset
impl From<SqliteRow> for Asset {
    fn from(val: SqliteRow) -> Self {
        Asset {
            asset_id: val.try_get("asset_id").unwrap_or_default(),
            name: val.try_get("name").unwrap_or_default(),
            location: val.try_get("location").unwrap_or_default(),
        }
    }
}

#[allow(dead_code)]
pub enum Status {
    NotSet = 0,
    Online = 1,
    Purge = 2,
    Deleted = 3,
}
impl Default for Status {
    fn default() -> Self {
        Status::NotSet
    }
}

#[derive(sqlx::FromRow, Debug, Clone, Default)]
pub struct Version {
    pub asset_id: i64,
    pub version_id: i64,
    pub version: i64,
    pub source: String,
    pub datapath: String,
    pub depend: String,
    pub approved: u8,
    pub status: u8,
    pub ctime: String,
    pub atime: String,
}

// Sqlite results into Version
impl From<&SqliteRow> for Version {
    fn from(row: &SqliteRow) -> Version {
        Version {
            asset_id: row.try_get("asset_id").unwrap_or_default(),
            version_id: row.try_get("asset_id").unwrap_or_default(),
            version: row.try_get("version").unwrap_or_default(),
            source: row.try_get("source").unwrap_or_default(),
            datapath: row.try_get("datapath").unwrap_or_default(),
            depend: row.try_get("depend").unwrap_or_default(),
            approved: row.try_get("approved").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            ctime: row.try_get("ctime").unwrap_or_default(),
            atime: row.try_get("atime").unwrap_or_default(),
        }
    }
}
