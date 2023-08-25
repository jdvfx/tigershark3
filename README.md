# Tigershark3

Tigershark3 is a cli based 3d asset versions tracking tool with simple CRUD functions<br>
using Rust+Sqlite3 backend and Python3 for the Houdini frontend.<br>
Since it's cli based and using JSON, it can be used with any software, not just Houdini.

**Dependencies**

* Rust https://www.rust-lang.org/tools/install
* build-essential
    * Ubuntu/Debian: sudo apt install build-essential
    * Fedora: sudo dnf group install "C Development Tools and Libraries"
* SQLite3
* Python3
* Houdini19 (optional)

**Build+Setup (Linux)**
build rust executable
```
$cargo build --release
```
setup executable and Sqlite3 DB (Tigershark exec path,DB path/name)<br>

adds two environment variables to .bashrc:<br>
* $PYTHONPATH for Houdini, the tigershark python module<br>
* $TS_DATABASE the Sqlite3 database path<br> 
```
$bash setup.sh
```


**CLI Syntax**

tigershark3 -c {command} -a {asset}

available commands:<br>
* insert : create new version (create if asset if )<br>
* latest : returns the latest version of the asset<br>
* source : returns the source file that created the current version<br>
* delete (mark version for deletion)<br>
* approve (approve asset version and dependencies)<br>
* purge : write text file listing all versions to delete<br>

asset format<br>
* Json<br>
    {"name":"my_asset","location":"myasset_location"}


**Examples**

* insert new asset<br>
./tigershark3 -c insert -a '{"name":"my_asset","location":"myasset_location"}'

* update asset<br>
./tigershark3 -c insert -a '{"name":"my_asset","location":"myasset_location","datapath":"/data/myasset","source":"/sources/myasset_source"}'

* find latest version of an asset<br>
./tigershark3 -c latest -a '{"name":"my_asset","location":"myasset_location"}'



**required Jason fields for each command**

* insert<br>
name && location || asset_id && datapath && source

* source<br>
name && location && version || asset_id && version || version_id

* delete<br>
name && location || asset_id && version || version_id

* latest<br>
name && location || asset_id

* approve<br>
name && location || asset_id && version || version_id

* purge<br>
// no Json required


### DB Storage Scheme (SQlite3)

> 2 tables (assets/versions)

### Assets<br>
asset_id	: i64 *<br>
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
asset_id	: i64 *<br>
ctime		: String<br>
atime		: String<br>
