use std::{fs::File, io::BufReader};

use actix::prelude::{Actor, Addr, StreamHandler};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::web::Data;
use actix_web::Error;
use actix_web::{
    http::header::ContentType, middleware, web, App, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws::{self, ProtocolError};
use log::debug;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::time::Instant;
use uuid::Uuid;

mod server;
mod constants;
pub use self::server::*;
pub use self::constants::*;

/// simple handle
async fn index(req: HttpRequest) -> HttpResponse {
    debug!("{req:?}");

    HttpResponse::Ok().content_type(ContentType::html()).body(
        "<!DOCTYPE html><html><body>\
            <p>Welcome to your TLS-secured homepage!</p>\
            <p>test <a href='static/ws.html'>websockets</a></p>
        </body></html>",
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // https://stackoverflow.com/questions/73255421/actix-web-requested-application-data-is-not-configured-correctly-view-enable-d
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let config = load_rustls_config();
    let ws_server = Server::new().start();

    log::info!("starting HTTPS server at https://localhost:8443");

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // Enable CORS for all origins
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials(),
            )
            // WebSocket route
            // inject ws_server in context
            .app_data(Data::new(ws_server.clone()))
            // webSockets: TRICK /ws/ route must be before / and others to prevent problems
            .route("/ws/", web::get().to(ws_index))
            // register simple handler, handle all methods
            .service(web::resource("/index.html").to(index))
            .service(web::redirect("/", "/index.html"))
            .service(Files::new("/static", "static"))
    })
    .bind_rustls_021("0.0.0.0:8443", config)?
    .workers(1)
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

/// Define your WebSocket session actor
pub struct MyWebSocketSession {
    id: String,
    hb: Instant,
    server_addr: Addr<Server>,
}

impl MyWebSocketSession {
    fn new(server_addr: Addr<Server>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            hb: Instant::now(),
            server_addr,
        }
    }
}

impl Actor for MyWebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
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
