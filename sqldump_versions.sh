if [ x"${TS_DATABASE_URL}" == "x" ]; then
	echo TS_DATABASE_URL environment variable not set
else 
	echo TS_DATABASE_URL: $TS_DATABASE_URL 
	sqlite3 $TS_DATABASE_URL "SELECT * FROM versions"
fi

