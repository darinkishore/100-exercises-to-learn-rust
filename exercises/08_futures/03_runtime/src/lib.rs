extern crate core;

// TODO: Implement the `fixed_reply` function. It should accept two
// `TcpListener` instances,  accept connections on both of them concurrently,
// and always reply clients by sending  the `Display` representation of the
// `reply` argument as a response.
use std::fmt::Display;
use std::sync::Arc;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::task::JoinSet;

async fn handle_connection<T>(mut socket: TcpStream, reply: Arc<T>)
    where
        T: Display + Send + Sync + 'static,
{
    if let Err(e) = socket.write_all(reply.to_string().as_bytes()).await {
        eprintln!("Error writing to socket: {}", e);
    }
}

pub async fn fixed_reply<T>(
    first: TcpListener,
    second: TcpListener,
    reply: T,
) where
    T: Display + Send + Sync + 'static,
{
    let repl = Arc::new(reply);

    let mut join_set = JoinSet::new();

    let repl1 = Arc::clone(&repl);
    join_set.spawn(async move {
        loop {
            match first.accept().await {
                Ok((socket, _)) => {
                    let repl_clone = repl1.clone();
                    tokio::spawn(async move { handle_connection(socket, repl_clone).await });
                }
                Err(e) => {
                    eprintln!("Error accepting connection from first listener: {:?}", e);
                    break;
                }
            }
        }
    });

    let repl2 = Arc::clone(&repl);
    join_set.spawn(async move {
        loop {
            match second.accept().await {
                Ok((socket, _)) => {
                    let repl_clone = repl2.clone();
                    tokio::spawn(async move { handle_connection(socket, repl_clone).await });
                }
                Err(e) => {
                    eprintln!("Error accepting connection from second listener: {:?}", e);
                    break;
                }
            }
        }
    });

    while let Some(result) = join_set.join_next().await {
        if let Err(e) = result {
            eprintln!("Error in server task: {:?}", e);
        }
    }
}


#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::panic;

    use tokio::io::AsyncReadExt;
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
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr, ] {
                join_set.spawn(async move {
                    let mut socket =
                        tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _, ) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
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
