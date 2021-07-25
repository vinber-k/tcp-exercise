use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:33333")?;
    for _ in 0..10 {
        let mut input = String::new();
        // 输入字符串
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        // 数据传入stream
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");

        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        // 读取在buffer中的数据
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        println!("{}",
            str::from_utf8(&buffer).expect("Could not write buffer as string"));
        println!("");
    }
    Ok(())
}
