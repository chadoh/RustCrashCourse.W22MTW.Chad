pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is from {}", "Chad", "PA");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Chad", "PA", "code");

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Chad",
        activity = "onewheeling"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
