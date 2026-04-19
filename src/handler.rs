
use axum::{
    body::Body,
    http::{header},
    response::{Html, IntoResponse, Response},
};
use futures::stream; // Теперь заработает после cargo add futures
use tokio::time::{sleep, Duration};
use crate::scr::capture; // Убедись, что путь к функции захвата верный

use crate::{CONTROLLER_JS, INDEX_HTML, INDEX_JS};


pub async fn serve_index() -> impl IntoResponse{
    Html(INDEX_HTML)
}

pub async fn serve_js_main() -> impl IntoResponse{
    Response::builder()
        .header(header::CONTENT_TYPE, "application/javascript")
        .body(axum::body::Body::from(INDEX_JS))
        .unwrap()
}

pub async fn serve_js_controller() -> impl IntoResponse{
    Response::builder()
        .header(header::CONTENT_TYPE, "application/javascript")
        .body(axum::body::Body::from(CONTROLLER_JS))
        .unwrap()
}

pub async fn screen_stream_handler() -> impl IntoResponse {
    // Создаем бесконечный поток байтов
    let stream = stream::unfold((), |_| async {
        // Делаем захват кадра
        if let Some(frame_bytes) = capture().await {
            if frame_bytes.is_empty() { return Some((Ok(vec![]), ())); }
            // Формируем MJPEG блок
            let data = format!(
                "--frame\r\n\
                 Content-Type: image/jpeg\r\n\
                 Content-Length: {}\r\n\r\n",
                frame_bytes.len()
            );

            let mut chunk = data.into_bytes();
            chunk.extend(frame_bytes);
            chunk.extend(b"\r\n");

            // Ограничиваем частоту кадров (например, 20 FPS)
            sleep(Duration::from_millis(50)).await;

            Some((Ok::<_, std::convert::Infallible>(chunk), ()))
        } else {
            // Если ошибка захвата, подождем чуть-чуть и попробуем снова
            sleep(Duration::from_millis(100)).await;
            Some((Ok(vec![]), ()))
        }
    });

    Response::builder()
        .header("Content-Type", "multipart/x-mixed-replace; boundary=frame")
        .body(Body::from_stream(stream))
        .unwrap()
}



