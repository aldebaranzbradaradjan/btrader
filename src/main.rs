use btrader::trader::bTrader;
use std::env;

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();
  let trader: bTrader = bTrader::new(&args[1]);
  trader.run().await;
}
