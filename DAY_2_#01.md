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
# Иммутабельность и shadowing
$ cargo run
   Compiling calculator-basic v0.1.0 (/home/aaaaa/rust/calculator-basic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/calculator-basic`
6 1000 -42 3.14 true ℝ 500 6.4 z [1, 2, 3, 4]
