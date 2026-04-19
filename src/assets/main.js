// Ждем загрузки DOM, чтобы гарантированно найти элементы
document.addEventListener('DOMContentLoaded', () => {
    const screenImg = document.getElementById('screen-view');
    const toggleBtn = document.getElementById('toggle-btn');
    const refreshBtn = document.getElementById('refresh-btn');
    const statusText = document.getElementById('status');

    let isStreaming = true;

    // Функция обновления статуса
    const updateStatus = (text, isError = false) => {
        statusText.innerText = `Статус: ${text}`;
        statusText.style.color = isError ? "#ff4444" : "#888";
    };

    // Функция запуска/перезапуска потока
    const startStream = () => {
        // Добавляем уникальный параметр query string (?t=...), 
        // чтобы сбросить кэш браузера при каждом перезапуске
        const timestamp = Date.now();
        screenImg.src = `/screen?t=${timestamp}`;
        updateStatus("Трансляция активна");
    };

    // Логика кнопки Старт/Стоп
    toggleBtn.addEventListener('click', () => {
        if (isStreaming) {
            // Чтобы разорвать активное HTTP-соединение MJPEG, очищаем src
            screenImg.src = ""; 
            toggleBtn.innerText = "Запустить трансляцию";
            updateStatus("Остановлено");
        } else {
            startStream();
            toggleBtn.innerText = "Остановить трансляцию";
        }
        isStreaming = !isStreaming;
    });

    // Кнопка принудительного обновления
    refreshBtn.addEventListener('click', () => {
        startStream();
        isStreaming = true;
        toggleBtn.innerText = "Остановить трансляцию";
    });

    // Обработка ошибок (например, если сервер упал)
    screenImg.onerror = () => {
        updateStatus("Ошибка захвата. Проверьте сервер Rust.", true);
    };

    // Инициализация при загрузке страницы
    startStream();
});





