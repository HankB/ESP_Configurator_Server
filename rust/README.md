# Rust ESP Configuration Server

Warning - it has "been a while" since I wrote something in Rust and the result is likely to demonstrate that. Start with

```text
rustup update
```

Result is

```text
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server/rust$ rustc --version
rustc 1.85.1 (4eb161250 2025-03-15)
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server/rust$ cargo --version
cargo 1.85.1 (d73d2caf9 2024-12-31)
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server/rust$ 
```

## 2025-03-20 References

* <https://doc.rust-lang.org/std/net/index.html>
* <https://docs.rs/socket2/latest/socket2/struct.Socket.html>

Port - From <https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml?search=Unassigned&page=2> `5016` is unassigned.

```text
mkdir rust
cd rust
cargo init rust
cargo run
```

```text
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server$ cargo new rust
    Creating binary (application) `rust` package
error: destination `/home/hbarta/Programming/ESP_Conf/ESP_Configurator_Server/rust` already exists

Use `cargo init` to initialize the directory
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server$ cd rust
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server/rust$ cargo init
    Creating binary (application) package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server/rust$ cargo run
   Compiling rust v0.1.0 (/home/hbarta/Programming/ESP_Conf/ESP_Configurator_Server/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/rust`
Hello, world!
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server/rust$ tree
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        ├── deps
        │   ├── rust-4fab5a36f4f59dcd
        │   └── rust-4fab5a36f4f59dcd.d
        ├── examples
        ├── incremental
        │   └── rust-1q4y2fl4cvu7n
        │       ├── s-h5o0wunzri-06824j6-bfc3uul6zg2byi3slvu3opzau
        │       │   ├── 77o7pb0zy0kkghgbbk9xqs4g6.o
        │       │   ├── 7f9i1a1wjy6cabpspnh3uipvu.o
        │       │   ├── 8e66bl61k0tlen89cnjzrqxww.o
        │       │   ├── bulot9oexzxby95l28wroc1jz.o
        │       │   ├── czi1od40j45uyt5ms3ot3vwzk.o
        │       │   ├── dep-graph.bin
        │       │   ├── dkuc06fck0vk546xofu8duocf.o
        │       │   ├── query-cache.bin
        │       │   └── work-products.bin
        │       └── s-h5o0wunzri-06824j6.lock
        ├── rust
        └── rust.d

10 directories, 19 files
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server/rust$ 
```

OK! Now start with some sample code from <https://doc.rust-lang.org/std/net/struct.UdpSocket.html>

## 2029-03-20 test

Using `netcat` (ref: <https://stackoverflow.com/questions/13294893/broadcasting-a-message-using-nc-netcat>)

```text
echo -n "request-config|b827eb4f1eb7" | nc -u -b -q 0 255.255.255.255 5016
```

## 2029-03-21 test with ESP

With the ESP coded to broacdcast the configuration request, the rust server has been tested to confirm that it receives the message.
