use std::time::Duration;
use std::{fs::File, io::BufReader};
use actix::prelude::Actor;
use actix::spawn;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::rt::time;
use actix_web::web::Data;
use actix_web::{
    http::header::ContentType, middleware, web, App, HttpRequest, HttpResponse, HttpServer,};
use log::{debug, error};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde_json::json;

mod constants;
mod socket_server;
pub use self::constants::*;
pub use self::socket_server::*;

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
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let config = load_rustls_config();
    let socket_server = Server::new().start();

    let ws_server_spawn = socket_server.clone();
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        let mut i = -1;
        loop {
            i += 1;
            interval.tick().await;
            // do something
            let msg_type = &format!("{}", String::from("echo"))[..];
            let json = json!({ "message": format!("hello message: #{}", i) });
            let message_to_client = MessageToClient::new(msg_type, json);
            match ws_server_spawn.send(message_to_client).await {
                Ok(_) => {
                    debug!("socket server sent message #{} to client", i);
                }
                Err(e) => error!("{:?}", e),
            };
        }
    });

    log::info!("starting HTTPS server at https://localhost:8443");

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    // .allowed_origin("http://localhost:5173")
                    // Enable CORS for all origins
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials(),
            )
            // WebSocket route
            // inject ws_server in context
            .app_data(Data::new(socket_server.clone()))
            // webSockets: TRICK /ws/ route must be before / and others to prevent problems
            .route("/ws/", web::get().to(ws_index))
            // register simple handler, handle all methods
            .service(web::resource("/index.html").to(index))
            // redirect to index
            .service(web::redirect("/", "/index.html"))
            // static files
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
