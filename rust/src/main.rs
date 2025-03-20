use std::net::UdpSocket;

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
        println!("amt:{} src:{} s:{}", amt, src, s);

    } // the socket is closed here
    Ok(())
}
