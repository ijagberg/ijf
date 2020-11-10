use async_std::net::TcpStream;
use async_std::prelude::*;
use std::{error::Error, io::Read};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct SenderOpts {
    #[structopt(long)]
    address: String,
    #[structopt(long)]
    port: u16,
    #[structopt(required = true)]
    file: String,
}

impl SenderOpts {
    fn address(&self) -> &str {
        &self.address
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn file(&self) -> &str {
        &self.file
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

        self.send_file().await?;

        Ok(())
    }

    async fn send_file(&self) -> Result<(), Box<dyn Error>> {
        let mut file = std::fs::File::open(self.opts.file())?;

        let mut buffer = Vec::new();

        file.read(&mut buffer)?;

        let mut stream = TcpStream::connect(&self.opts.formatted_address()).await?;
        
        println!("sending {} bytes", buffer.len());
        let timer = std::time::Instant::now();

        stream.write_all(&buffer).await?;

        println!("took {:?}", timer.elapsed());

        Ok(())
    }
}
