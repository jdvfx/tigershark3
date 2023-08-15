# tigershark3

Tigershark3 is a CLI based Houdini asset version tracking tool with simple CRUD functions using SQLite3, Python3 (Houdini front-end) and Rust (back-end). Since it's CLI based, it can be used with any software, not just Houdini.

# CLI Syntax

tigershark3 -c {command} -a {asset}

available commands:
- insert : create new version (create if asset if )
- latest : returns the latest version of the asset
- source : returns the source file that created the current version
- delete (mark version for deletion)
- approve (approve asset version and dependencies)
- purge : write text file listing all versions to delete

asset format
- Json
    {"name":"my_asset","location":"myasset_location"}


# Examples

insert new asset
./tigershark3 -c insert -a '{"name":"my_asset","location":"myasset_location"}'

update asset
./tigershark3 -c insert -a '{"name":"my_asset","location":"myasset_location","datapath":"/data/myasset","source":"/sources/myasset_source"}'

find latest version of an asset
./tigershark3 -c latest -a '{"name":"my_asset","location":"myasset_location"}'



# required Jason fields for each command

insert<br>
name && location || asset_id && datapath && source

source<br>
name && a_location || a_asset_id && a_version || a_version_id

delete<br>
name && a_location || a_asset_id && a_version || a_version_id

latest<br>
a_name && a_location || a_asset_id

approve<br>
name && a_location || a_asset_id && a_version || a_version_id

purge<br>
// no Json required





# DB Storage Scheme (SQlite3)

> 2 tables (assets/versions)

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


