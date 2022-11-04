pub fn run() {
    let mut hello = "Hello ".to_string();

    // Get length
    let len = hello.len();

    // push a char
    hello.push('W');

    // push a string
    hello.push_str("orld");

    let capacity = hello.capacity();

    let is_empty = hello.is_empty();

    let contains = hello.contains("World");

    hello = hello.replace("World", "There");

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    println!("{:?}", (hello, len, capacity, is_empty, contains));

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
