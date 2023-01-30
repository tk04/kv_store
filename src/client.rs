use std::io::prelude::Write;
use std::net::TcpStream;
pub fn send() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:6000")?;
    stream.write(b"set newV 0 0 5\r\nhello\r\n")?;
    stream.write(b"get newV\r\n")?;
    stream.write(b"get newV\r\n")?;
    stream.write(b"get newV\r\n")?;
    stream.write(b"get newV\r\n")?;

    Ok(())
}
