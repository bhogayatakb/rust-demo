pub fn run() {
    let name = "Keval";
    let mut age = 23;
    println!("My name is {}, & age is {}", name, age);
    age = 24;
    println!("My name is {}, & age is {}", name, age);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let (name1, age1) = ("Karan", "45");
    println!("His name is {}, & age is {}", name1, age1);
}