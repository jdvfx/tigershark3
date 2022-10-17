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
