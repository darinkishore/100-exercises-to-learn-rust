use std::panic;

use tokio::net::TcpListener;
use tokio::try_join;

// TODO: write an echo server that accepts TCP connections on two listeners,
// concurrently.  Multiple connections (on the same listeners) should be
// processed concurrently.  The received data should be echoed back to the
// client.
pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<()> {
    loop {
        let (mut socket1, _) = first.accept().await?;
        let task1 = tokio::spawn(async move {
            let (mut reader, mut writer) = socket1.split();
            tokio::io::copy(&mut reader, &mut writer).await.map_err(|_| anyhow::anyhow!("Failed to echo on first listener"))
        });

        let (mut socket2, _) = second.accept().await?;
        let task2 = tokio::spawn(async move {
            let (mut reader, mut writer) = socket2.split();
            tokio::io::copy(&mut reader, &mut writer).await.map_err(|_| anyhow::anyhow!("Failed to echo on second listener"))
        });

        try_join!(task1, task2)?;
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::panic;

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::task::JoinSet;

    use super::*;

    async fn bind_random() -> (TcpListener, SocketAddr, ) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr, )
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr, ) = bind_random().await;
        let (second_listener, second_addr, ) = bind_random().await;
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in requests.clone() {
            for addr in [first_addr, second_addr, ] {
                join_set.spawn(async move {
                    let mut socket =
                        tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer, ) = socket.split();

                    // Send the request
                    writer.write_all(request.as_bytes()).await.unwrap();
                    // Close the write side of the socket
                    writer.shutdown().await.unwrap();

                    // Read the response
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
                }, );
            }
        }

        while let Some(outcome, ) = join_set.join_next().await {
            if let Err(e, ) = outcome {
                if let Ok(reason, ) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
