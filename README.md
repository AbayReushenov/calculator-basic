## README.md

# calculator-basic

CLI-калькулятор на Rust (День 2 учебного плана). Поддерживает операции: add, sub, mul, div. Реализованы проверки ввода и обработка ошибок (некорректные числа, деление на ноль).

## Возможности

- Операции: add, sub, mul, div.
- Парсинг аргументов командной строки.
- Коды выхода: 1 — неверное число аргументов, 2 — ошибка парсинга числа, 3 — деление на ноль.


## Установка

Требуется установленный Rust (rustup, cargo) и настроенный PATH.

- Клонировать/создать проект
- Сборка:

```bash
cargo build
```


## Запуск

Формат:

```bash
cargo run -- <op> <a> <b>
```

Где:

- op: add | sub | mul | div
- a, b: числа (f64)

Примеры:

```bash
cargo run -- add 2 3       # 5
cargo run -- sub 10 7      # 3
cargo run -- mul 6 4       # 24
cargo run -- div 10 2      # 5
```

Ошибочные случаи:

```bash
cargo run -- div 10 0      # сообщение + код выхода 3
cargo run -- add x 3       # сообщение об ошибке парсинга + код выхода 2
cargo run -- add 1         # usage + код выхода 1
```


## Пример кода (основной вариант без внешних зависимостей)

```rust
use std::env;

fn usage() {
    eprintln!("Usage: calc <op> <a> <b>");
    eprintln!("Ops: add | sub | mul | div");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        usage();
        std::process::exit(1);
    }

    let op = &args[^1];

    let a: f64 = args[^2].parse().unwrap_or_else(|_| {
        eprintln!("Invalid number: {}", args[^2]);
        std::process::exit(2);
    });

    let b: f64 = args[^3].parse().unwrap_or_else(|_| {
        eprintln!("Invalid number: {}", args[^3]);
        std::process::exit(2);
    });

    let result = match op.as_str() {
        "add" => a + b,
        "sub" => a - b,
        "mul" => a * b,
        "div" => {
            if b == 0.0 {
                eprintln!("Division by zero");
                std::process::exit(3);
            }
            a / b
        }
        _ => {
            usage();
            std::process::exit(1);
        }
    };

    println!("{result}");
}
```


## Тесты

Запуск:

```bash
cargo test
```

Пример модульных тестов (внутри модуля с логикой):

```rust
pub mod ops {
    pub fn add(a: f64, b: f64) -> f64 { a + b }
    pub fn sub(a: f64, b: f64) -> f64 { a - b }
    pub fn mul(a: f64, b: f64) -> f64 { a * b }
    pub fn div(a: f64, b: f64) -> Result<f64, &'static str> {
        if b == 0.0 { Err("division by zero") } else { Ok(a / b) }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test] fn add_ok() { assert_eq!(add(2.0, 3.0), 5.0); }
        #[test] fn div_by_zero_err() { assert!(div(1.0, 0.0).is_err()); }
    }
}
```

Рекомендация по структуре:

- Для небольшого CLI — хранить тесты рядом с кодом.
- При росте — вынести вычислительную логику в `src/lib.rs`, а интеграционные тесты — в `tests/`.


## Структура проекта

```
calculator-basic/
├── Cargo.toml
└── src/
    └── main.rs
```

Опционально при масштабировании:

```
src/
├── lib.rs        # операции и бизнес‑логика
└── main.rs       # CLI: парсинг аргументов, вывод
tests/
└── integration_cli.rs
```


## Качество кода

Полезные команды:

```bash
cargo check      # быстрый анализ
cargo fmt        # форматирование
cargo clippy     # статический анализ
```

Для диагностики паник:

```bash
RUST_BACKTRACE=1 cargo run -- <op> <a> <b>
```


## Расширения (по желанию)

- Дополнительные операции: mod, pow, sqrt.
- REPL‑режим (интерактивный ввод выражений).
- Поддержка целочисленного режима и настроек округления.
- Переключение локали/языка сообщений.
- Автоматическая справка через clap.


## Лицензия

MIT

## Команды для коммита

```bash
git init
git add .
git commit -m "Day 2: calculator-basic CLI with usage and tests"
git branch -M main
git remote add origin https://github.com/<username>/calculator-basic.git
git push -u origin main
```


Запуск на любой машине той же ОС без Cargo:

Собрать релиз: cargo build --release
```bash
  cargo build --release
```

Скопировать только файл target/release/calculator-basic и запускать его напрямую (двойной клик или через терминал).

