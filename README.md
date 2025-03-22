# ESP_Configurator_Server

Provide the server component of the ESP_Configurator.

## 2025-03-19 Description

Provide a utility that:

1. Listens for configuration requests broadcast via UDP.
1. Matches the MAC address provided in the request with configuration entries in an SQLite database.
1. Sends the configuration entries back to the client via UDP.

## 2025-03-19 Protocol

In general this will be a text based protocol with fields delimited by `|` character as something not likely to be found in the actual data fields. Messages need not be terminated with a newline (`\n` in `C`) or null.

The configuration request will look like:

```text
request-config|b827eb4f1eb7
```

`b827eb4f1eb7` is an example of a device MAC address. A series of replies will look like

```text
nn|b827eb4f1eb7|sensor1|location|parm
nn|b827eb4f1eb7|hostname|name
nn|b827eb4f1eb7|broker|IP
nn|b827eb4f1eb7|sensor2|location|parm1|parm2
```

Where

* nn - decimal length of the message, formatted as ASCII , inclusive of all fields. The length is artificially constrained to 99 characters to limit the RAM requirements in the ESP.
* b827eb4f1eb7 - MAC address of the device that requested the settings, formatted as ASCII.
* sensor, hostname, location, parm... - Various configuration values. The number of these varies so from the standpoint of the server, they are treated as a single entry to be sent to and parsded by the ESP.

## 2025-03-19 Language

Almost any language which supports access to network communications and as Sqlite database could be used. This could be easily accomplished using a bash script that shells out to`netcat`, `sqlite3` and `awk`, `sed`, grep or similar to parse the messages. A more interesting language might be `golang` or `Rust`. To entertain the notion that the implementatin language could vary, code will be produced in a subdirectory, starting with `rust`.

## 2025-03-21 Database design

KISS - the DB will consist of a single table with columns consisting of the text fields `MAC` and `config`. The application will select the rows that match the MAC address and package the message with length, `MAC` and `config` and send to the ESP requesting configuration. 

```text
sudo apt install -y sqlite3
```

Referring to <https://www.sqlite.org/quickstart.html> and <https://www.sqlite.org/cli.html> since it has been too long since I've worked with a proper database.

