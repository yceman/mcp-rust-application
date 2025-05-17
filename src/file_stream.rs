use tokio::net::TcpStream;
//use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn::error:Error>> {
    let addr = "127.0.0.1:8081";
    let mut stream = TcpStream::connect(addr).awayt?;
    println!("Listen on port...", addr);
}
