use actix_web::{web, HttpResponse};
use crate::ws::chat_ws_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(|| async { HttpResponse::Ok().body("Chat Server Running") }));
    cfg.route("/ws/", web::get().to(chat_ws_handler));
}
