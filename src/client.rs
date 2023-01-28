use std::io::prelude::Write;
use std::net::TcpStream;
pub fn send() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:6000")?;
    stream.write(b"set newV 1024 \r\nthis is the value \r\n")?;

    Ok(())
}
