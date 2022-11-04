pub fn run() {
    greeting("Hello", "Jane");

    let get_sum = add(5, -3);
    println!("Sum: {}", get_sum);

    // Closure
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C Sum: {}", add_nums(n3, 5));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name)
}

fn add(n1: i8, n2: i8) -> i8 {
    n1 + n2
}
