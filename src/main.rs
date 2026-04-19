#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use axum::{Router, routing::get,routing::post};
use crate::handler::screen_stream_handler;
use crate::handler::serve_index;
use crate::handler::serve_js_controller;
use crate::handler::serve_js_main;
use crate::controller::move_mouse_handler;

mod handler;
mod scr;
mod controller;

const INDEX_HTML: &[u8] = include_bytes!("assets/index.html");
const INDEX_JS: &[u8] = include_bytes!("assets/main.js");
const CONTROLLER_JS: &[u8] = include_bytes!("assets/controller.js");



#[tokio::main]
async fn main(){

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/main.js", get(serve_js_main))
        .route("/controller.js", get(serve_js_controller))
        .route("/screen", get(screen_stream_handler))
        .route("/mouse",post(move_mouse_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await.unwrap();
    
    if cfg!(debug_assertions){
        println!("Запущен в дебаг режиме");
        println!("Запущен сервер на: 0.0.0.0:1234");
    } 
    

    axum::serve(listener, app).await.unwrap();
    
}



