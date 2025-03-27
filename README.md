# WikiGame - Игра по поиску пути в Википедии

![Docker](https://img.shields.io/badge/Docker-✓-blue)
![Rust](https://img.shields.io/badge/Rust-✓-orange)

WikiGame - это консольная игра на Rust, которая находит путь между двумя статьями Википедии, переходя по ссылкам.

## 🚀 Быстрый старт

### Сборка и запуск через Docker Compose

1. Склонируйте репозиторий:

```bash
git clone https://github.com/yourusername/wikigame.git
cd wikigame
```

2. Запустите приложение:

```bash
docker-compose up --build
```

### Настройка параметров

Вы можете изменить параметры запуска через переменные окружения в `docker-compose.yml`:

```yaml
environment:
  - START_ARTICLE=Python
  - END_ARTICLE=Artificial intelligence
```

Или передать аргументы при запуске:

```bash
docker-compose run wikigame --start "Python" --end "Machine learning"
```

## 🔧 Конфигурация

Файл `config.toml` позволяет настроить:

```toml
[wiki]
language = "en"       # Язык Википедии (en, ru, fr и т.д.)
max_iterations = 1000 # Максимальное количество шагов поиска
pllimit = 500         # Лимит ссылок в запросе
lhlimit = 500         # Лимит обратных ссылок в запросе
using_date = true     # Использоветь даты
```

## 📂 Структура проекта

```
wikigame/
├── src/               # Исходный код
│   ├── main.rs        # Точка входа
│   ├── api.rs         # Работа с API Википедии
│   ├── game.rs        # Логика игры
│   ├── cli.rs         # Логика cli
│   ├── ulit.rs        # Разные фукции для фильтрации
│   └── config.rs      # Конфигурация
├── Cargo.toml         # Зависимости Rust
├── Dockerfile         # Конфигурация Docker
├── docker-compose.yml # Конфигурация Docker Compose
└── config.toml        # Настройки приложения
```

## 🛠 Разработка

### Локальная сборка

1. Установите Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Соберите проект:

```bash
cargo build --release
```

3. Запустите:

```bash
./target/release/wikigame --start "Rust" --end "Programming"
```

4. Быстрый запуск:

```bash
cargo run --release -- --start "Rust" --end "Programming"
```

## 🌐 Доступные команды

```
--start, -s    Стартовая статья (необязательно)
--end, -e      Целевая статья (необязательно)
--verbose, -v  Уровень детализации логов (можно указать несколько раз)
```

## 📝 Примеры использования

1. Поиск пути между статьями:

```bash
docker-compose run wikigame --start "Python" --end "Artificial intelligence"
```

2. Запуск с подробным логом:

```bash
docker-compose run wikigame -s "Rust" -e "Systems" -vv
```

## 📜 Лицензия

MIT License. См. файл [LICENSE](LICENSE).