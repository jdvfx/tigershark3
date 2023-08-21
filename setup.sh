#!/bin/bash
ts_path_default="$HOME/tigershark"
db_path_default="$HOME/tigershark_db"
db_name_default="ts1"

echo Tigershark install path
echo [default] $ts_path_default
read TS_PATH
echo DB path
echo [default] $db_path_default
read DB_PATH
echo DB name
echo [default] $db_name_default
read DB_NAME

if [ -z "$TS_PATH" ]
then
	TS_PATH=$ts_path_default
fi
if [ -z "$DB_PATH" ]
then
	DB_PATH=$db_path_default
fi
if [ -z "$DB_NAME" ]
then
	DB_NAME=$db_name_default
fi

# add the .db extension
DB_NAME=$DB_NAME.db

echo ts_path=$TS_PATH
echo db_path=$DB_PATH
echo db_name=$DB_NAME

echo --------------------------------
read -p "Do you want to proceed? (yes/no) " yn

case $yn in 
	yes ) echo . . . ;;
	no ) echo exiting...;
		exit;;
	* ) echo invalid response;
		exit 1;;
esac

mkdir -p $TS_PATH
mkdir -p $DB_PATH
cp -r * $TS_PATH
echo $TS_PATH > ~/.tigershark_db_path
sqlite3 $DB_PATH/$DB_NAME ".read db/schema.sql"

echo install config saved in: ~/.tigershark_db_path
echo DB created: $DB_PATH/$DB_NAME
echo TS_DATABASE_URL=sqlite:$DB_PATH/$DB_NAME > .env

echo TS_PATH added to .bashrc
echo 'export PYTHONPATH=$PYTHONPATH:'$TS_PATH'/python' >> ~/.bashrc
