use std::net::UdpSocket;
use rusqlite;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("0.0.0.0:5016")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 100];
        let (amt, src) = socket.recv_from(&mut buf)?;
        
        let s = match std::str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("amt:{} src:{} s:{}\n", amt, src, s);
        let parts = s.split("|");
        //for part in parts {
            //println!("{}", part)
        //}
        let collection = parts.collect::<Vec<&str>>();
        dbg!(&collection);
        println!("\n{:?}\n", collection[1]);
        
        let connection = rusqlite::open("../DB/config.db").unwrap();
        use rusqlite::State;

        let query = "SELECT * FROM ESP_config WHERE ESP_config = ?";

        let mut statement = connection.prepare(query).unwrap();
        statement.bind((1, collection[1])).unwrap();
        
        while let Ok(State::Row) = statement.next() {
            println!("MAC = {}", statement.read::<String, _>("MAC").unwrap());
            println!("config = {}", statement.read::<i64, _>("config").unwrap());
        }
    } // the socket is closed here
    Ok(())
}
