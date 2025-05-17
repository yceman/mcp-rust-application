//use tokio::net::TcpStream;
//use tokio::io::{AsyncWriteExt, AsyncReadExt};
use mini_redis::{ client, Result };
#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:6379";
    let mut client = client::connect(addr).await?;
    client.set("Hello", "World".into()).await?;

    let result = client.get("Hello").await?;
    println!("Got value from the server...; result={:?}", result);
    Ok(())
}
