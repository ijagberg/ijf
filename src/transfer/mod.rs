mod receive;
mod send;

use std::error::Error;

pub use receive::{Receiver, ReceiverOpts};
pub use send::{Sender, SenderOpts};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct TransferOpts {
    #[structopt(subcommand)]
    command: Command,
}

impl TransferOpts {
    pub fn command(&self) -> &Command {
        &self.command
    }
}

#[derive(Debug, StructOpt, Clone)]
pub enum Command {
    Send(SenderOpts),
    Receive(ReceiverOpts),
}

#[derive(Debug)]
pub struct Transferer {
    opts: TransferOpts,
}

impl Transferer {
    pub fn new(opts: TransferOpts) -> Self {
        Self { opts }
    }

    pub async fn transfer(self) -> Result<(), Box<dyn Error>> {
        match self.opts.command() {
            Command::Send(send_opts) => {
                let sender = Sender::new(send_opts.clone());
                sender.send().await?;
            }
            Command::Receive(receiver_opts) => {
                let receiver = Receiver::new(receiver_opts.clone());
                receiver.receive().await?;
            }
        }

        Ok(())
    }
}
