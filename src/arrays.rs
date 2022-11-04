use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Re-assign value
    numbers[2] = 20;

    println!("Single value: {}", numbers[0]);

    println!("Array length: {}", numbers.len());

    // Arrays are stack-allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}
