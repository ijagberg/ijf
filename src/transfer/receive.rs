use async_std::net::TcpStream;
use async_std::prelude::*;
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
        let mut stream = TcpStream::connect(&self.opts.formatted_address()).await?;

        let mut buffer = vec![0u8; 1024];
        let amount: usize = stream.read(&mut buffer).await?;

        println!("received {} bytes", amount);

        let content = String::from_utf8(buffer)?;

        println!("content: '{}'", content);

        Ok(())
    }
}
