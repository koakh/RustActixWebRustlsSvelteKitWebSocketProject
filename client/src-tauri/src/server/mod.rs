pub mod handlers;

use actix::prelude::Actor;
use actix_cors::Cors;
use actix_web::{
  dev::ServiceRequest,
  middleware,
  rt::{spawn, time},
  web::{self, Data},
  App, Error as ActixError, HttpServer,
};
use actix_web_httpauth::{
  extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
  },
  middleware::HttpAuthentication,
};
use log::{debug, error, info};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde_json::json;
use std::{
  cell::Cell,
  env,
  sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
  },
};
use std::{fs::File, io::BufReader, time::Duration};
use tauri::AppHandle;

use crate::{
  websocket::{SocketServer, ws_index},
  app::constants::*,
  MessageToClientType,
  websocket::MessageToClient
};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct TauriAppState {
  app: Mutex<AppHandle>,
}

#[actix_web::main]
pub async fn init(app: AppHandle) -> std::io::Result<()> {
  // env vars
  let http_server_uri = env::var("HTTP_SERVER_URI").unwrap_or_else(|_| DEFAULT_HTTP_SERVER_URI.to_string());
  let https_server_uri = env::var("HTTPS_SERVER_URI").unwrap_or_else(|_| DEFAULT_HTTPS_SERVER_URI.to_string());
  let https_server_enable_https = env::var("HTTPS_SERVER_ENABLE_HTTPS").unwrap_or_else(|_| DEFAULT_HTTPS_SERVER_ENABLE_HTTPS.to_string());
  let http_server_api_key = env::var("HTTP_SERVER_API_KEY").unwrap_or_else(|_| DEFAULT_HTTP_SERVER_API_KEY.to_string());
  let log_actixweb_middleware_format = env::var("LOG_ACTIXWEB_MIDDLEWARE_FORMAT").unwrap_or_else(|_| DEFAULT_LOG_ACTIXWEB_MIDDLEWARE_FORMAT.to_string());
  let spawn_thread_enabled = env::var("SPAWN_THREAD_ENABLED").unwrap_or_else(|_| DEFAULT_SPAWN_THREAD_ENABLED.to_string());
  // fingerprint
  let machine_uid: String = machine_uid::get().unwrap();

  // tauri state
  let tauri_app = web::Data::new(TauriAppState { app: Mutex::new(app) });

  // the trick for not lost connections sessions, is create socket_server outside of HttpServer::new, and use `move ||`
  let socket_server = SocketServer::new().start();
  
  // if spawn_thread_enabled == "true" {
    let socket_server_spawn = socket_server.clone();
    let mut i = -1;
    spawn(async move {
      let mut interval = time::interval(Duration::from_secs(DEFAULT_SPAWN_THREAD_DURATION_SECONDS));
      debug!("start spawn interval SPAWN_THREAD_DURATION_SECONDS: {}", DEFAULT_SPAWN_THREAD_DURATION_SECONDS);
      loop {
        i += 1;
        interval.tick().await;
        let msg_type = &format!("{}", MessageToClientType::Echo)[..];
        let json = json!({ "message": format!("hello message: #{}", i) });
        let message_to_client = MessageToClient::new(msg_type, json);
        match socket_server_spawn.send(message_to_client).await {
          Ok(_) => {}
          Err(e) => error!("{:?}", e),
        };
      }
    });

  // authentication validator
  async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, ActixError> {
    let http_server_api_key = env::var("HTTP_SERVER_API_KEY").unwrap_or_else(|_| DEFAULT_HTTP_SERVER_API_KEY.to_string());
    if credentials.token() == http_server_api_key.to_string() {
      Ok(req)
    } else {
      let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default)
        .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");
      error!("{}", "invalid authorization api key".to_string());
      Err(AuthenticationError::from(config).into())
    }
  }

  // clones to use in ws http server
  let log_actixweb_middleware_format_ws = log_actixweb_middleware_format.clone();
  let socket_server_ws = socket_server.clone();

  // http_ws server, always used for frontend websockets comunication
  let config = load_rustls_config();
  let http_server = HttpServer::new(move || {
    // TODO:
    // cors: allowed_origin(http_vite_server.as_str())
    let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method().supports_credentials();
    App::new()
      .wrap(cors)
      // enable logger
      .wrap(middleware::Logger::new(log_actixweb_middleware_format_ws.as_str()))
      // inject socket_server in context
      .app_data(Data::new(socket_server_ws.clone()))
      // webSockets: TRICK /ws/ route must be before / and others to prevent problems
      .service(web::resource("/ws/").route(web::get().to(ws_index)))
  })
  // .workers(2)
  .keep_alive(Duration::from_secs(HTTP_SERVER_KEEP_ALIVE))
  .bind(http_server_uri)?
  .run();

  let https_server = HttpServer::new(move || {
    // cors
    // TODO:
    let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method().supports_credentials();
    App::new()
      .wrap(cors)
      // enable logger
      .wrap(middleware::Logger::new(log_actixweb_middleware_format.as_str()))
      // new actixweb MUST USE everything wrapped in Data::new() this is the solution for websockets connection error
      // .app_data(Data::new(AppState {
      //   server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
      //   request_count: Cell::new(0),
      //   // filter,
      // }))
      // global data: don't wrap it in data::new() it's already wrapped above
      // TODO:
      //.app_data(data.clone())
      // inject socket_server in context
      .app_data(Data::new(socket_server.clone()))
      // webSockets: TRICK /ws/ route must be before / and others to prevent problems
      .service(web::resource("/ws/").route(web::get().to(ws_index)))
      // TODO:
      // .service(health_check)
      // .service(health_check_identity_server)
      // .service(redirect)
  })
  // .workers(2)
  .keep_alive(Duration::from_secs(HTTP_SERVER_KEEP_ALIVE))
  .bind_rustls_021(https_server_uri, config)?
  .run();
  
  // run servers concurrently
  futures::try_join!(http_server, https_server).map(|_| ())  
}

fn load_rustls_config() -> rustls::ServerConfig {
  // init server config builder with safe defaults
  let config = ServerConfig::builder().with_safe_defaults().with_no_client_auth();
  // load TLS key/cert files
  let mut cert_file = BufReader::new(CERT_CERT_FILE.as_bytes());
  let mut key_file = BufReader::new(CERT_KEY_PEM.as_bytes());
  // convert files to key/cert objects
  let cert_chain = certs(&mut cert_file).unwrap().into_iter().map(Certificate).collect();
  let mut keys: Vec<PrivateKey> = pkcs8_private_keys(&mut key_file).unwrap().into_iter().map(PrivateKey).collect();
  // exit if no keys could be parsed
  if keys.is_empty() {
    eprintln!("Could not locate PKCS 8 private keys.");
    std::process::exit(1);
  }
  config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
