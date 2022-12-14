# tigershark3

Tigershark3 is a CLI based Houdini asset version tracking tool with simple CRUD functions using SQLite3, Python3 (Houdini front-end) and Rust (back-end). Since it's CLI based, it can be used with any software, not just Houdini.

- insert (create/update)
- get_latest (latest version)
- get_source (path of file that created the asset)
- delete (tag for deletion, a separate tool does the actual deletion)
- approve (approve asset version and dependencies)

syntax:<br>

> insert new asset<br>
./tigershark3 -c insert -a '{"name":"my_asset","location":"myasset_location"}'

> update asset<br>
./tigershark3 -c insert -a '{"name":"my_asset","location":"myasset_location","datapath":"/data/myasset","source":"/sources/myasset_source"}'

> find latest version of an asset<br>
./tigershark3 -c latest -a '{"name":"my_asset","location":"myasset_location"}'

> 2 SQLite tables (assets/versions)

### Assets<br>
asset_id	: i64<br>
name		: String<br>
location	: String<br>

### Versions<br>
version_id	: i64<br>
version		: i64<br>
source		: String<br>
datapath	: String<br>
depend		: String<br>
approved	: i64<br>
status		: i64<br>
asset_id	: i64<br>
ctime		: String<br>
atime		: String<br>


