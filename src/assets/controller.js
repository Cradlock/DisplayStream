let moveInterval = null;
const speedInput = document.getElementById('speed-range');
const speedVal = document.getElementById('speed-val');

// Обновляем текст при движении ползунка
speedInput.oninput = () => speedVal.innerText = speedInput.value;

async function sendMove(dx, dy) {
    const step = parseInt(speedInput.value);
    try {
        await fetch('/mouse', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ dx: dx * step, dy: dy * step }),
        });
    } catch (e) { console.error("Ошибка сети"); }
}

// Функция старта движения
window.startMoving = function(dx, dy, event) {
    if (event) event.preventDefault();
    if (moveInterval) return;

    // Сразу делаем один шаг
    sendMove(dx, dy);

    // Запускаем цикл (каждые 60мс — это примерно 15 команд в секунду)
    moveInterval = setInterval(() => sendMove(dx, dy), 60);
};

// Функция остановки
window.stopMoving = function(event) {
    if (event) event.preventDefault();
    clearInterval(moveInterval);
    moveInterval = null;
};
