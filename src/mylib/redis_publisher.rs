extern crate redis;

use sdt::error::Error;
use redis::Commands;
use crate::message::Message;

pub fn publish_message(message: Message) -> Result<(), Box<dyn Error>> {
  let client: Client = redis::Client::open("redis//localhost/")?;
  let mut con: Connection = client.get_connection()?;

  let json: String = serde_json::to_string(message)?;

  con.publish(message.channel, json)?;

  Ok(())
}
