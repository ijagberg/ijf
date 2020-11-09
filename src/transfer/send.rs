use async_std::net::TcpStream;
use async_std::prelude::*;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct SenderOpts {
    #[structopt(long)]
    address: String,
    #[structopt(long)]
    port: u16,
    #[structopt(required = true)]
    files: Vec<String>,
}

impl SenderOpts {
    fn address(&self) -> &str {
        &self.address
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn files(&self) -> &Vec<String> {
        &self.files
    }

    fn formatted_address(&self) -> String {
        format!("{}:{}", self.address(), self.port())
    }
}

pub struct Sender {
    opts: SenderOpts,
}

impl Sender {
    pub fn new(opts: SenderOpts) -> Self {
        Self { opts }
    }

    pub async fn send(self) -> Result<(), Box<dyn Error>> {
        println!("sending with opts: {:#?}", self.opts);
        let mut stream = TcpStream::connect(&self.opts.formatted_address()).await?;
        stream.write_all(b"Hello!").await?;
        stream.write_all(b"Helloo!").await?;

        Ok(())
    }
}
