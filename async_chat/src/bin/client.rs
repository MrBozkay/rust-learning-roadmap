use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() {
    let socket = TcpStream::connect("127.0.0.1:8090").await.unwrap();
    println!("Connected to server.");
    
    let (reader, mut writer) = socket.into_split();

    tokio::spawn(async move {
        let mut reader = BufReader::new(reader);
        let mut line = String::new();
        loop {
            let bytes = reader.read_line(&mut line).await.unwrap();
            if bytes == 0 { return; }
            print!("{}", line);
            line.clear();
        }
    });

    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut line = String::new();
    loop {
        stdin.read_line(&mut line).await.unwrap();
        writer.write_all(line.as_bytes()).await.unwrap();
        line.clear();
    }
}
