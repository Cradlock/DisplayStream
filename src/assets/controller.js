
// Делаем функцию доступной глобально
window.moveMouse = async function(dx, dy, event) {
    // Останавливаем стандартное поведение (важно для мобилок)
    if (event) {
        event.preventDefault();
    }

    try {
        await fetch('/mouse', { // Убедись, что путь совпадает с .route в Rust
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ dx, dy }),
        });
    } catch (err) {
        console.error("Ошибка управления:", err);
    }
};


