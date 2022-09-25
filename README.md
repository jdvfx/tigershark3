# tigershark3
tigershark v2 but moved from MongoDB to SQLITE3,
and adding Assets creation as well as Show/Seq/Shot

Tigershark3
is a CLI based 3D asset version tracking tool with simple CRUD functions using SQLite

- insert (create/update)
- get_latest (latest version)
- get_source (path of file that created the asset)
- delete (tag for deletion, a separate tool does the actual deletion)
- approve (approve asset version and dependencies)

syntax:<br>

> insert new asset<br>
./tigershark2 -c insert -a '{"name":"my_asset","location":"myasset_location"}'

> update asset<br>
./tigershark2 -c insert -a '{"name":"my_asset","location":"myasset_location","datapath":"/data/myasset","source":"/sources/myasset_source"}'

> find latest version of an asset<br>
./tigershark2 -c latest -a '{"name":"my_asset","location":"myasset_location"}'

> 2 SQLite tables (assets/versions)

> Assets
asset_id	: i64
name		: String
location	: String

> Versions
version_id	: i64
version		: i64
source		: String
datapath	: String
depend		: String
approved	: i64
status		: i64
asset_id	: i64
ctime		: String
atime		: String

