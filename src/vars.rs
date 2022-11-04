pub fn run() {
    let name = "Chad";
    let mut age = 34;
    println!("My name is {} and I am {}", name, age);
    age = 35;
    println!("My name is {} and I am {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Chad", 35);
    println!("{} is {}", my_name, my_age);
}
