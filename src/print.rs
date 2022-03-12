pub fn run() {
    // print to console
    println!("Hello from print.rs file");

    // basic formatting
    println!("Number: {}", 1);

    // positional argument
    println!(
        "{0} {2} {1}", "zero", "one", "two"
    );

    //name argument
    println!("{name} eats {food}", name = "Keval", food = "Roti");

    //placeholder traits
    println!("binary {:b}, Hex {:x}, Octal {:o}", 12, 12, 12);

    //placeholder for debug trait
    println!("{:?}", (10, "anything", false));

    //basic math
    println!("10 + 10 = {}", 10 + 10)


}