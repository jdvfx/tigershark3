source set_db_env.sh
../target/debug/tigershark2 -c create -a '{
"name":"Iron3",
"location":"my_iron_location",
"source":"source_that_created_iron",
"depend":["bob","joe","bill"],
"datapath":"/my/data/path/my_iron"}'

