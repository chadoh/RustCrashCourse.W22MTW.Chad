pub fn run() {
    let x = 1;

    let y = 2.5;

    let z: i64 = 1987234792374018234;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    let a1 = '🥸';

    println!("{:?}", (x, y, z, is_active, is_greater, a1));
}
