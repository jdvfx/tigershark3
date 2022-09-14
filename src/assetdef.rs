use sqlx::sqlite::SqliteRow;
use sqlx::Row;

#[derive(sqlx::FromRow, Debug)]
pub struct Version {
    pub asset_id: i64,
    pub version_id: i64,
    pub version: i64,
    pub source: String,
    pub datapath: String,
    pub depend: String,
    pub approved: u8,
    pub status: u8,
}
impl From<&SqliteRow> for Version {
    fn from(row: &SqliteRow) -> Version {
        Version {
            asset_id: row.try_get("asset_id").unwrap_or(0),
            version_id: row.try_get("asset_id").unwrap_or(0),
            version: row.try_get("version").unwrap_or(0_i64),
            source: row.try_get("source").unwrap_or("_".to_string()),
            datapath: row.try_get("datapath").unwrap_or("_".to_string()),
            depend: row.try_get("depend").unwrap_or("_".to_string()),
            approved: row.try_get("approved").unwrap_or(0),
            status: row.try_get("status").unwrap_or(0),
        }
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Asset {
    pub asset_id: i64,
    pub name: String,
    pub location: String,
}
impl From<&SqliteRow> for Asset {
    fn from(row: &SqliteRow) -> Asset {
        Asset {
            asset_id: row.try_get("asset_id").unwrap_or(0_i64),
            name: row.try_get("name").unwrap_or("_".to_string()),
            location: row.try_get("location").unwrap_or("_".to_string()),
        }
    }
}
