use sqlx::sqlite::SqliteRow;
use sqlx::Row;

#[derive(sqlx::FromRow, Debug)]
pub struct Version {
    pub id: i64,
    pub source: String,
    pub datapath: String,
    pub depend: String,
    pub approved: u8,
    pub status: u8,
    pub asset_id: i64,
}
impl From<&SqliteRow> for Version {
    fn from(row: &SqliteRow) -> Version {
        Version {
            id: row.try_get("id").unwrap_or(0_i64),
            source: row.try_get("source").unwrap_or("_".to_string()),
            datapath: row.try_get("datapath").unwrap_or("_".to_string()),
            depend: row.try_get("depend").unwrap_or("_".to_string()),
            approved: row.try_get("approved").unwrap_or(0),
            status: row.try_get("status").unwrap_or(0),
            asset_id: row.try_get("asset_id").unwrap_or(0),
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Asset {
    pub id: i64,
    pub name: String,
}
impl From<&SqliteRow> for Asset {
    fn from(row: &SqliteRow) -> Asset {
        Asset {
            id: row.try_get("id").unwrap_or(0_i64),
            name: row.try_get("name").unwrap_or("_".to_string()),
        }
    }
}
