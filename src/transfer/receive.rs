use async_std::prelude::*;
use async_std::{net::TcpListener, task};
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct ReceiverOpts {
    #[structopt(long)]
    port: u16,
}

impl ReceiverOpts {
    fn formatted_address(&self) -> String {
        format!("localhost:{}", self.port)
    }
}

pub struct Receiver {
    opts: ReceiverOpts,
}

impl Receiver {
    pub fn new(opts: ReceiverOpts) -> Self {
        Self { opts }
    }

    pub async fn receive(self) -> Result<(), Box<dyn Error>> {
        println!("receiving with opts: {:#?}", self.opts);
        let listener = TcpListener::bind(&self.opts.formatted_address()).await?;

        loop {
            // Asynchronously wait for an inbound socket.
            let (mut socket, _) = listener.accept().await?;

            // And this is where much of the magic of this server happens. We
            // crucially want all clients to make progress concurrently, rather than
            // blocking one on completion of another. To achieve this we use the
            // `tokio::spawn` function to execute the work in the background.
            //
            // Essentially here we're executing a new task to run concurrently,
            // which will allow all of our clients to be processed concurrently.

            task::spawn(async move {
                let mut buf = vec![0; 1024];
                loop {
                    let bytes = socket
                        .read(&mut buf)
                        .await
                        .expect("failed to read data from socket");
                    if bytes == 0 {
                        break;
                    }
                }

                if let Ok(content) = String::from_utf8(buf) {
                    println!("content: '{}'", content);
                } else {
                    println!("invalid utf8");
                }
            });
        }
    }
}
