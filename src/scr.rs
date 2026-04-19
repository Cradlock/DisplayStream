
use enigo::{Enigo, Mouse};
use xcap::Monitor;
use std::io::Cursor;
use image::{DynamicImage, ImageFormat, RgbaImage, Rgba};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

fn draw_cursor_overlay(image: &mut RgbaImage) {
    // 1. Получаем координаты мыши (относительно всего экрана X11)
    let settings = enigo::Settings::default();
    let Ok(enigo) = Enigo::new(&settings) else { return; };
    let Ok((mouse_x, mouse_y)) = enigo.location() else { return; };

    // 2. ВАЖНО: Если ты скринишь только ОКНО, нужно вычесть координаты окна!
    // Если скринишь весь МОНИТОР, этот шаг пропусти.
    // let (win_x, win_y) = browser_window.x(), browser_window.y();
    // let draw_x = mouse_x - win_x;
    // let draw_y = mouse_y - win_y;
    let draw_x = mouse_x; // Для полного экрана
    let draw_y = mouse_y;

    // 3. Рисуем яркий квадрат (например, красный с белой обводкой)
    let cursor_size = 10;
    let color_red = Rgba([255, 0, 0, 255]);
    
    // Рисуем заполненный прямоугольник по координатам мыши
    draw_filled_rect_mut(
        image,
        Rect::at(draw_x as i32, draw_y as i32).of_size(cursor_size, cursor_size),
        color_red,
    );
}

pub async fn capture() -> Option<Vec<u8>> {
    let result = tokio::task::spawn_blocking(|| {
        let monitors = Monitor::all().ok()?;
        
        let monitor = monitors.into_iter().next()?;

        let mut image = monitor.capture_image().ok()?;

        let mut buffer = Cursor::new(Vec::new());
        
        draw_cursor_overlay(&mut image);

        let dynamic_image = DynamicImage::ImageRgba8(image);
        
        let res_image = dynamic_image.resize(
            dynamic_image.width() / 2,
            dynamic_image.height() / 2,
            image::imageops::FilterType::Triangle 
        );
        
        res_image.write_to(&mut buffer, ImageFormat::Jpeg).ok()?;
        Some(buffer.into_inner())
    })
    .await;

    match result {
        Ok(res) => res,
        Err(e) => {
            eprintln!("DEBUG: Ошибка потока: {:?}", e);
            None
        }
    }
}



