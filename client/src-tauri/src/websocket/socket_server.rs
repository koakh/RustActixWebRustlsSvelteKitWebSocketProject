use actix::prelude::{Actor, Context, Handler, Message as ActixMessage, Recipient};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{error::Result as SerdeResult, to_string, Value};
use std::collections::HashMap;

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(ActixMessage, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct MessageToClient {
  pub msg_type: String,
  pub data: Value,
}

impl MessageToClient {
  pub fn new(msg_type: &str, data: Value) -> Self {
    Self { msg_type: msg_type.to_string(), data }
  }
}

pub struct SocketServer {
  sessions: HashMap<String, Recipient<Message>>,
}

impl SocketServer {
  pub fn new() -> Self {
    SocketServer { sessions: HashMap::new() }
  }

  fn send_message(&self, data: SerdeResult<String>) {
    match data {
      Ok(data) => {
        for recipient in self.sessions.values() {
          recipient.do_send(Message(data.clone()))
        }
      }
      Err(err) => {
        format!("Data did not convert to string {:?}", err);
      }
    }
  }
}

impl Actor for SocketServer {
  type Context = Context<Self>;
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Connect {
  pub addr: Recipient<Message>,
  pub id: String,
}

impl Handler<Connect> for SocketServer {
  type Result = ();

  fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
    self.sessions.insert(msg.id.clone(), msg.addr);
    debug!("handle connection: id: {}, sessionsLen: {}", msg.id.clone(), self.sessions.len());
  }
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Disconnect {
  pub id: String,
}

impl Handler<Disconnect> for SocketServer {
  type Result = ();

  fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
    debug!("handle disconnection: id: {}", msg.id.clone());
    self.sessions.remove(&msg.id);
  }
}

impl Handler<MessageToClient> for SocketServer {
  type Result = ();

  fn handle(&mut self, msg: MessageToClient, _: &mut Context<Self>) -> Self::Result {
    debug!("handle messageToClient: sessionsLen: {}", self.sessions.len());
    self.send_message(to_string(&msg));
  }
}
