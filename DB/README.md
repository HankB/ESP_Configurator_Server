# SQLite database

This is more or less a place to explore the database and ways to manipulate it, including some ad-hoc scripts to populate it.

First off, create the database:

```text
echo "CREATE TABLE ESP_config(MAC text, config text);" | sqlite3 config.db
```

Add some data

```text
cat <<EOF | sqlite3 config.db
INSERT INTO ESP_config VALUES('b827eb4f1eb7','DS18B20|28d5275600000049|main level');
INSERT INTO ESP_config VALUES('b827eb4f1eb7','hostname|cheshire');
EOF
```

And verify

```text
echo "select * from ESP_config;" | sqlite3 config.db
```

```text
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server/DB$ echo "select * from ESP_config;" | sqlite3 config.db
b827eb4f1eb7|DS18B20|28d5275600000049|main level
b827eb4f1eb7|hostname|cheshire
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server/DB$ 
```

It's interesting that `sqlite3` uses the same separator by default `|` as was chosen for the config portion of the message.
