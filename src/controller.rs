
use axum::{http::StatusCode, response::IntoResponse, Json};
use enigo::{Coordinate, Enigo, Mouse, Settings}; // Импортируем Mouse и Settings
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MouseMove {
    dx: i32,
    dy: i32,
}

pub async fn move_mouse_handler(Json(payload): Json<MouseMove>) -> impl IntoResponse {
    // Используем spawn_blocking, так как взаимодействие с ОС — синхронное
    tokio::task::spawn_blocking(move || {
        // 1. Создаем настройки по умолчанию
        let settings = Settings::default();

        // 2. Инициализируем Enigo и распаковываем Result
        if let Ok(mut enigo) = Enigo::new(&settings) {
            // 3. В новой версии используется метод move_mouse
            // Coordinate::Rel означает "относительно текущей позиции"
            let _ = enigo.move_mouse(payload.dx, payload.dy, Coordinate::Rel);
        } else {
            if cfg!(debug_assertions){
                eprintln!("Не удалось инициализировать Enigo");
            }
        }
    })
    .await
    .ok();

    StatusCode::OK
}

