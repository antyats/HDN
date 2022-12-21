use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
/// Sends client request
/// Load request:
/// {
///   "request_type": "load",
///   "key": "some_key"
/// }
/// Store request:
/// {
/// "request_type": "store",
/// "key": "some_key",
/// "hash": "0b672dd94fd3da6a8d404b66ee3f0c83"
/// }
///

fn main() -> io::Result<()> {
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:7777")?;

    let mut request = String::new();
    io::stdin().read_line(&mut request)?;

    stream.write_all(request.as_bytes())?;

    let mut reader = BufReader::new(&mut stream);

    let mut buffer = vec![];
    reader
        .read_until(b'}', &mut buffer)
        .expect("Fail of getting response from server");
    let parsed_string = std::str::from_utf8(&buffer).unwrap();

    println!("Server response: {:?}", parsed_string);

    Ok(())
}
