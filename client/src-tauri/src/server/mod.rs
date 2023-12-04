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

use crate::websocket::SocketServer;
use crate::app::constants::*;

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct TauriAppState {
  app: Mutex<AppHandle>,
}

#[actix_web::main]
pub async fn init(app: AppHandle) -> std::io::Result<()> {
  // env vars
  let http_server_uri = env::var("HTTP_SERVER_URI").unwrap_or_else(|_| DEFAULT_HTTP_SERVER_URI.to_string());
  let http_server_enable_https = env::var("HTTP_SERVER_ENABLE_HTTPS").unwrap_or_else(|_| DEFAULT_HTTP_SERVER_ENABLE_HTTPS.to_string());
  let cert_file_name_key = env::var("CERT_FILE_NAME_KEY").unwrap_or_else(|_| DEFAULT_CERT_FILE_NAME_KEY.to_string());
  let cert_file_name_cert = env::var("CERT_FILE_NAME_CERT").unwrap_or_else(|_| DEFAULT_CERT_FILE_NAME_CERT.to_string());
  let http_server_api_key = env::var("HTTP_SERVER_API_KEY").unwrap_or_else(|_| DEFAULT_HTTP_SERVER_API_KEY.to_string());
  let log_actixweb_middleware_format = env::var("LOG_ACTIXWEB_MIDDLEWARE_FORMAT").unwrap_or_else(|_| DEFAULT_LOG_ACTIXWEB_MIDDLEWARE_FORMAT.to_string());
  let spawn_thread_enabled = env::var("SPAWN_THREAD_ENABLED").unwrap_or_else(|_| DEFAULT_SPAWN_THREAD_ENABLED.to_string());
  // fingerprint
  let machine_uid: String = machine_uid::get().unwrap();

  // tauri state
  let tauri_app = web::Data::new(TauriAppState { app: Mutex::new(app) });

  // the trick for not lost connections sessions, is create ws_server outside of HttpServer::new, and use `move ||`
  let ws_server = SocketServer::new().start();
  
  // spawn loop in parallel thread with async
  // if spawn_thread_enabled == "true" {
  //   let ws_server_spawn = ws_server.clone();
  //   spawn(async move {
  //     let mut interval = time::interval(Duration::from_secs(DEFAULT_SPAWN_THREAD_DURATION_SECONDS));
  //     debug!("start spawn interval SPAWN_THREAD_DURATION_SECONDS: {}", DEFAULT_SPAWN_THREAD_DURATION_SECONDS);
  //     loop {
  //       interval.tick().await;
  //       // get health check message from identity server
  //       let ping_message = match identity_server_ping().await {
  //         Ok(v) => v,
  //         Err(err) => MessageResponse { message: err.to_string() },
  //       };

  //       // do something
  //       let json = json!({ "message": format!("identity server health check response: '{}'", ping_message.message) });
  //       // let wsm: WebSocketMessage = serde_json::from_value(json).unwrap();
  //       let msg_type = &format!("{}", MessageToClientType::HealthCheckIdentityServerApiResponse)[..];
  //       let message_to_client = MessageToClient::new(msg_type, json);
  //       // let message_to_client = MessageToClient::new("echo", json);
  //       // websocket_srv.do_send(message_to_client);
  //       match ws_server_spawn.send(message_to_client).await {
  //         Ok(_) => {}
  //         Err(e) => error!("{:?}", e),
  //       };
  //     }
  //   });
  // }

  let mut current_config_file = String::from("");
  let data = web::Data::new(AppStateGlobal {
    counter: Mutex::new(0),
    config_file: Arc::new(Mutex::new(Some(current_config_file))),
    citizen_card_state: Arc::new(Mutex::new(None)),
    machine_uid: Arc::new(Mutex::new(machine_uid.clone())),
  });

  // authentication validator
  // required to implement ResponseError in src/app/errors.rs else we have a error
  // Err(AuthenticationError::from(config).into())
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

  let http_server = HttpServer::new(move || {
    // cors
    let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method().supports_credentials();
    App::new()
      .wrap(cors)
      // enable logger
      .wrap(middleware::Logger::new(log_actixweb_middleware_format.as_str()))
      // new actixweb MUST USE everything wrapped in Data::new() this is the solution for websockets connection error
      .app_data(Data::new(AppState {
        server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
        request_count: Cell::new(0),
        // filter,
      }))
      // global data: don't wrap it in data::new() it's already wrapped above
      .app_data(data.clone())
      // inject ws_server in context
      .app_data(Data::new(ws_server.clone()))
      // webSockets: TRICK /ws/ route must be before / and others to prevent problems
      .service(web::resource("/ws/").route(web::get().to(ws_index)))
      .service(health_check)
      .service(health_check_identity_server)
      .service(redirect)
  })
  // .workers(2)
  .keep_alive(Duration::from_secs(HTTP_SERVER_KEEP_ALIVE));

  if http_server_enable_https.eq("true") {
    info!(
      "start {} rest server at: '{}', apiKey: '{}...', certificates '{}', '{}'",
      APP_NAME,
      http_server_uri,
      &http_server_api_key[..10],
      cert_file_name_key,
      cert_file_name_cert
    );

    // New ActixWeb Rustls Way
    let config = load_rustls_config();
    http_server.bind_rustls_021(http_server_uri, config)?.run().await
  } else {
    info!("start {} rest server at: '{}', apiKey: '{}...'", APP_NAME, http_server_uri, &http_server_api_key[..10]);
    // start server
    http_server.bind(http_server_uri)?.run().await
  }
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
