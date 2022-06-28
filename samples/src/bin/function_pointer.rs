type F = fn(i32, i32) -> (u8, i32);

fn execute_all(x: i32, y: i32, funcs: &[F]) {
    for func in funcs {
        let (op, res) = func(x, y);
        println!("{} {} {} = {}", x, op as char, y, res);
    }
}

fn add(x: i32, y: i32) -> (u8, i32) {
    (b'+', x + y)
}

fn sub<T: std::ops::Sub<Output = T>>(x: T, y: T) -> (u8, T) {
    (b'-', x - y)
}

fn main() {
    let mul = |x: i32, y: i32| (b'*', x * y);

    let div = |x, y| (b'/', x / y);

    let func_vec: Vec<F> = vec![add, sub::<i32>, mul, div];
    execute_all(10, 20, &func_vec);

    let func_arr: [F; 4] = [add, sub::<i32>, mul, div];
    execute_all(3, 2, &func_arr);
}
