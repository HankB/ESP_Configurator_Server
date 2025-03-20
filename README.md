# ESP_Configurator_Server

Provide the server component of the ESP_Configurator.

## 2025-03-19 Description

Provide a utility that:

1. Listens for configuration requests broadcast via UDP.
1. Matches the MAC address provided in the request with configuration entries in an Sqlite database.
1. Sends the configuration entries back to the client via UDP.

## 2025-03-19 Protocol

In general this will be a text based protocol with fields delimited by `|` character as something not likely to be found in the actual data fields. Messages need not be terminated with a newline (`\n` in `C`) or null.

The configuration request will look like:

```text
request-config|b827eb4f1eb7
```

`b827eb4f1eb7` is an example of a device MAC address. A series of replies will look like

```text
nnn|b827eb4f1eb7|key|value
nnn|b827eb4f1eb7|key|value
nnn|b827eb4f1eb7|key|value
```

Where

* nnn - decimal length of the message, formatted as ASCII , inclusive of all three fields.
* b827eb4f1eb7 - MAC address of the device that requested the settings, formatted as ASCII.
* key - text string identifing a configuration value such as `sensor`, `broker`, `area` etc.
* value - text string representing the value of the configuration item such as `DS18B20`, `spartan`, `main level` and so on.

## 2025-03-19 Language

Almost any language which supports access to network communications and as Sqlite database could be used. This could be easily accomplished using a bash script that shells out to`netcat`, `sqlite3` and `awk`, `sed`, grep or similar to parse the messages. A more interesting language might be `golang` or `Rust`. To entertain the notion that the implementatin language could vary, code will be produced in a subdirectory, starting with `rust`.
