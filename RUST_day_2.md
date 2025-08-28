# ИЗУЧЕНИЕ RUST

Дай подробную инструкцию что делать по данному сценарию

```
День 2: Переменные и типы данных
Immutable/mutable переменные
Скалярные и составные типы
Проект: calculator-basic - консольный калькулятор
```

Короткий ответ: во второй день нужно отработать неизменяемые/изменяемые переменные и типы данных Rust, а затем сделать консольный проект calculator-basic с парсингом аргументов и арифметикой; ниже пошаговая инструкция с кодом, проверками и коммитом в GitHub.[^1][^2][^3]

## Шаг 1: Теория и быстрые примеры

- Переменные по умолчанию неизменяемые; делаются изменяемыми через ключевое слово mut; полезны также константы const и «затенение» shadowing.[^4][^1]
- Скалярные типы: целые, числа с плавающей точкой, булевы, символы; составные: кортежи и массивы; кортежи гетерогенные и фиксированной длины.[^5][^2]

Мини-пример:

```rust
fn main() {
    // Иммутабельность и shadowing
    let x = 5;
    let x = x + 1; // shadowing — новое связывание
    // let mut x = 5; x = 6; // альтернатива: mut

    // Константа
    const MAX_OPS: usize = 1000;

    // Скалярные
    let a: i32 = -42;
    let b: f64 = 3.14;
    let ok: bool = true;
    let ch: char = 'ℝ';

    // Составные
    let tup: (i32, f64, char) = (500, 6.4, 'z');
    let (i, f, c) = tup; // деструктуризация
    let arr: [i32; 4] = [1, 2, 3, 4];

    println!("{x} {MAX_OPS} {a} {b} {ok} {ch} {i} {f} {c} {:?}", arr);
}
```

Этот пример демонстрирует mut, shadowing, константы, скалярные и составные типы.[^2][^1][^5]

## Шаг 2: Создать проект calculator-basic

```bash
cargo new calculator-basic
cd calculator-basic
```

Структура:

```
calculator-basic/
├── Cargo.toml
└── src/
    └── main.rs
```

Создание через cargo формирует каркас CLI-приложения.[^3]

## Шаг 3: Реализовать парсинг аргументов

Вариант A: стандартная библиотека (минимум зависимостей):

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

    let op = &args[^11];
    let a: f64 = args[^12].parse().unwrap_or_else(|_| {
        eprintln!("Invalid number: {}", args[^12]);
        std::process::exit(2);
    });
    let b: f64 = args[^13].parse().unwrap_or_else(|_| {
        eprintln!("Invalid number: {}", args[^13]);
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

Этот скелет реализует простой калькулятор с обработкой ошибок и делением на ноль.[^3]

Вариант B: удобный CLI-парсер (clap):

- В Cargo.toml:

```
[dependencies]
clap = { version = "4", features = ["derive"] }
```

- В src/main.rs:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "calc", version, about = "CLI калькулятор на Rust")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { a: f64, b: f64 },
    Sub { a: f64, b: f64 },
    Mul { a: f64, b: f64 },
    Div { a: f64, b: f64 },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Add { a, b } => a + b,
        Command::Sub { a, b } => a - b,
        Command::Mul { a, b } => a * b,
        Command::Div { a, b } => {
            if b == 0.0 { eprintln!("Division by zero"); std::process::exit(3); }
            a / b
        }
    };
    println!("{result}");
}
```


Clap даст автогенерируемую помощь и валидацию аргументов.[^3]

## Шаг 4: Тесты и модульность

Вынесите операции в модуль и покройте тестами:

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

Тесты оформляются через \#[cfg(test)] и cargo test.[^2]

## Шаг 5: Сборка, запуск, проверка

- Сборка/запуск:

```bash
cargo run -- add 2 3
cargo run -- div 10 2
cargo run -- div 10 0
```

- Проверка, форматирование, линт:

```bash
cargo check
cargo fmt
cargo clippy
```


Эти команды формируют привычный цикл разработки CLI в Rust.[^1][^2]

## Шаг 6: Расширения по желанию

- Поддержать целые и десятичные типы, выводить точность с форматированием, добавить операции pow/mod.[^5][^2]
- Сделать REPL-режим: читать выражение из stdin в бесконечном цикле, парсить и считать (начните с токенизации по пробелам).[^6][^3]
- Добавить README с примерами запуска, Makefile со сценариями build/test/run.[^3]


## Шаг 7: Коммит в GitHub

```bash
git init
git add .
git commit -m "Day 2: variables, types, calculator-basic CLI"
git branch -M main
git remote add origin https://github.com/<user>/calculator-basic.git
git push -u origin main
```

Пуш ежедневно ведет прогресс и формирует портфолио CLI-утилит.[^3]

## Чек-лист дня

- Понимание mut, const, shadowing; скалярных и составных типов.[^1][^2]
- Рабочий calc с обработкой ошибок и тестами; оформленный README; пуш в репозиторий.[^3]
<span style="display:none">[^10][^7][^8][^9]</span>

<div style="text-align: center">⁂</div>

[^1]: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

[^2]: https://doc.rust-lang.org/book/ch03-02-data-types.html

[^3]: https://lib.rs/crates/rust-cli-calculator

[^4]: https://www.woodruff.dev/variables-in-rust-let-me-be-immutable/

[^5]: https://dev.to/francescoxx/rust-data-types-1mlg

[^6]: https://github.com/stijnh/rust-calculator

[^7]: https://dev.to/stellaacharoiro/understanding-variables-and-mutability-in-rust-a-beginners-guide-3ifd

[^8]: https://internals.rust-lang.org/t/immutable-variable-is-an-absurd-term/21075

[^9]: https://stackoverflow.com/questions/63861322/is-this-the-correct-way-to-make-mutable-variable-immutable-again-in-rust

[^10]: https://dev.to/ahmed__elboshi/rust-tutorials-for-python-dev-variables-and-mutability-in-rust-4hpf

[^11]: https://sh.rustup.rs

[^12]: https://blog.jetbrains.com/rust/2024/09/20/how-to-learn-rust/

[^13]: https://web3.career/learn-web3/web3-developer-2025-roadmap

