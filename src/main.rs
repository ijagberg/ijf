mod transfer;

use async_std::task;
use structopt::StructOpt;
use transfer::*;

fn main() {
    dotenv::dotenv().ok();
    let opts = TransferOpts::from_args();

    let transferer = Transferer::new(opts);

    task::block_on(async move {
        let result = transferer.transfer().await;
        match result {
            Ok(ok) => {
                println!("success: {:?}", ok);
            }
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
    })
}
