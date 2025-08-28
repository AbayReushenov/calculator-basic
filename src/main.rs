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
