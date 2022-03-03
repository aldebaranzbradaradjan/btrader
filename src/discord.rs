use crate::config::Configuration;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use serenity::Error;
use serenity::{http::Http, model::id::ChannelId};

#[derive(Debug)]
pub struct DiscordBot {
  discord_token: String,
  discord_channel_id: u64,
  in_tx: Mutex<Sender<String>>,
  in_rx: Arc<Mutex<Receiver<String>>>,
}

impl DiscordBot {
  pub fn new(config: Configuration) -> DiscordBot {
    let (in_tx, in_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let in_tx_mutex = Mutex::new(in_tx);
    let in_rx_mutex = Arc::new(Mutex::new(in_rx));
    DiscordBot {
      discord_token: config.discord_token,
      discord_channel_id: config.discord_channel_id,
      in_tx: in_tx_mutex,
      in_rx: in_rx_mutex,
    }
  }
  pub fn send_message(&self, message: String) {
    if self.in_tx.lock().unwrap().send(message.clone()).is_err() {
      println!("Failed to send message \"{}\"", message);
    };
  }
  pub fn start(&self) {
    let client = Http::new_with_token(&self.discord_token);
    let channel = ChannelId(self.discord_channel_id);
    let in_rx_clone = self.in_rx.clone();
    thread::spawn(move || bot_main(client, channel, in_rx_clone));
  }
}

#[tokio::main]
async fn bot_main(
  client: Http,
  channel: ChannelId,
  in_rx: Arc<Mutex<Receiver<String>>>,
) -> Result<(), Error> {
  loop {
    if let Ok(message) = in_rx.lock().unwrap().try_recv() {
      channel
        .say(&client, message)
        .await ?;
    }
  }
}
