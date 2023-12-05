use actix::prelude::{Actor, Context, Handler, Message as ActixMessage, Recipient};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{error::Result as SerdeResult, to_string, Value};
use std::collections::HashMap;
use std::time::Instant;
use actix_web_actors::ws::{self, ProtocolError};
use uuid::Uuid;
use actix::prelude::{Addr, StreamHandler};
use actix_web::{Error, HttpRequest, HttpResponse, web};

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Debug, ActixMessage, Deserialize, Serialize)]
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

pub struct Server {
  sessions: HashMap<String, Recipient<Message>>,
}

impl Server {
  pub fn new() -> Self {
    Server { sessions: HashMap::new() }
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

impl Actor for Server {
  type Context = Context<Self>;
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Connect {
  pub addr: Recipient<Message>,
  pub id: String,
}

impl Handler<Connect> for Server {
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

impl Handler<Disconnect> for Server {
  type Result = ();

  fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
    debug!("handle disconnection: id: {}", msg.id.clone());
    self.sessions.remove(&msg.id);
  }
}

impl Handler<MessageToClient> for Server {
  type Result = ();

  fn handle(&mut self, msg: MessageToClient, _: &mut Context<Self>) -> Self::Result {
    debug!("handle messageToClient: sessionsLen: {}", self.sessions.len());
    self.send_message(to_string(&msg));
  }
}

/// Define your WebSocket session actor

pub struct MyWebSocketSession {
  _id: String,
  _hb: Instant,
  _server_addr: Addr<Server>,
}

impl MyWebSocketSession {
  fn new(server_addr: Addr<Server>) -> Self {
      Self {
          _id: Uuid::new_v4().to_string(),
          _hb: Instant::now(),
          _server_addr: server_addr,
      }
  }
}

impl Actor for MyWebSocketSession {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, _ctx: &mut Self::Context) {
      println!("WebSocket session started");
  }
}

impl StreamHandler<Result<ws::Message, ProtocolError>> for MyWebSocketSession {
  fn handle(&mut self, msg: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
      match msg {
          Ok(ws::Message::Ping(msg)) => {
              ctx.pong(&msg);
          }
          Ok(ws::Message::Text(text)) => {
              // Handle incoming text message
              ctx.text(text);
          }
          _ => (),
      }
  }
}

pub async fn ws_index(
  req: HttpRequest,
  stream: web::Payload,
  server_addr: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
  let res = ws::start(
      MyWebSocketSession::new(server_addr.get_ref().clone()),
      &req,
      stream,
  )?;

  Ok(res)
}
