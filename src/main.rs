use std::env;
use std::net::SocketAddr;

use tokio::{
    io,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> io::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|err| {
        eprintln!("'RUST_LOG' not set; {err:?}");
        EnvFilter::new("lucy=debug,tokio=error")
    });

    let formatter = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(filter)
        .with(formatter)
        .init();

    let addr = match env::args().nth(1) {
        Some(val) => val,
        None => "0.0.0.0:4567".into(),
    };

    let addr = addr
        .parse::<SocketAddr>()
        .expect("socket bind address is invalid");

    let listener = TcpListener::bind(&addr).await?;
    tracing::info!(
        "Service starting. Listening on: http://{}",
        listener.local_addr()?
    );

    // Main loop for accepting incoming connections
    loop {
        //
        let (mut socket, _) = listener.accept().await?;

        // Spawn a task to handle reading and writing from the stream/socket
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                // 1. Read from the socket
                // 2. Parse / decode bytes into meaningful data structures
                // 3. Do something interesting or useful with the data
                let n = match socket.read(&mut buf).await {
                    // Read successful, but no bytes read. Due to EOF
                    Ok(n) if n == 0 => return,

                    // Read successful, `n` bytes read
                    Ok(n) => {
                        tracing::info!("{n} bytes read from socket");
                        n
                    }

                    Err(err) => {
                        tracing::error!(?err, "failed to accept socket");
                        return;
                    }
                };

                // Encode and/or serialize data and write back to byte stream
                match socket.write_all(&buf[0..n]).await {
                    Ok(_) => {
                        let resp = std::str::from_utf8(&buf[0..n])
                            .expect("failed to encode buffer to UTF-8");
                        tracing::info!(resp = ?resp, "sent response")
                    }
                    Err(err) => {
                        tracing::error!(?err, "failed to write to socket");
                        return;
                    }
                };
            }
        });
    }
}
