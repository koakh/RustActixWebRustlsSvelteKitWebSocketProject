// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::File, io::BufReader};
use std::thread;
use log::{info, LevelFilter};
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
use tauri::Manager;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};

mod server;
mod websocket;
mod enums;
mod app;
pub use self::server::*;
pub use self::enums::*;

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::Webview];
#[cfg(debug_assertions)]
const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Info;
#[cfg(debug_assertions)]
const LOG_FILTER_MODULE: &str = "tauri_sidecar_cpp";

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];
#[cfg(not(debug_assertions))]
const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Warn;
#[cfg(not(debug_assertions))]
const LOG_FILTER_MODULE: &str = "tauri_sidecar_cpp";

// fn main() {
//     tauri::Builder::default()
//       .run(tauri::generate_context!())
//       .expect("error while running tauri application");
// }    

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
  info!("tauri: greet with arg: '{}'", name);
  format!("Hello, {}! You've been greeted from Rust!", name)
}

/// simple handle
async fn index(req: HttpRequest) -> HttpResponse {
    debug!("{req:?}");

    HttpResponse::Ok().content_type(ContentType::html()).body(
    r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>WebSocket Example</title>
    </head>
    <body>
    
    <script>
        const socket = new WebSocket('wss://localhost:8443/ws/');
    
        // Connection opened
        socket.addEventListener('open', (event) => {
            console.log('WebSocket connection opened:', event);
            socket.send('Hello, server!');
        });
    
        // Listen for messages
        socket.addEventListener('message', (event) => {
            console.log('Received message:', event.data);
        });
    
        // Connection closed
        socket.addEventListener('close', (event) => {
            console.log('WebSocket connection closed:', event);
        });
    
        // Connection error
        socket.addEventListener('error', (event) => {
            console.error('WebSocket error:', event);
        });
    </script>
    
    </body>
    </html>"#,
    )
}

fn main() {
    tauri::Builder::default()
      .setup(|app| {
        let window = app.get_window("main").unwrap();
        #[cfg(debug_assertions)]
        window.open_devtools();
  
        // AppHandle
        let handle = app.handle();
        // Box<AppHandle>
        let boxed_handle = Box::new(handle);
        // closure captures ownership of the boxed_handle variable using the move keyword, which moves the boxed handle into the closure
        thread::spawn(move || {
            server::init(*boxed_handle).unwrap();
        });
        Ok(())
      })
      // tauri log plugin
      .plugin(
        tauri_plugin_log::Builder::default()
          .targets(LOG_TARGETS)
          .with_colors(ColoredLevelConfig::default())
          .level(LOG_LEVEL_FILTER)
          .level_for(LOG_FILTER_MODULE, LOG_LEVEL_FILTER)
          .build(),
      )
      // This is where you pass in your commands
      .invoke_handler(tauri::generate_handler![greet])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }
