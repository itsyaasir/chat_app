use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            handle_connection(stream).await.unwrap();
        });
    }

    Ok(())
}

async fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut buf = [0; 1024];

    stream.read_exact(&mut buf).await?;

    let contents = fs::read_to_string("../../chat.html").await?;

    let response = format!("{}{}", "HTTPS/1.1 200 OK \r\n\r\n", contents);

    stream.write_all(response.as_bytes()).await?;

    stream.flush().await?;

    Ok(())
}
