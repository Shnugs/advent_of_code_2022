fn main() {
    println!("Hello, world!");

    println!("{}, {}!", "Hello", "world");
    println!("{0}, {1}!", "Hello", "world");
    println!("{greeting}, {name}!", greeting="Hello", name="world");

    let (greeting, name) = ("Hello", "world");
    println!("{greeting}, {name}!");

    println!("{:?}", [1,2,3]);
    println!("{:#?}", [1,2,3]);

    // `format!` macro used to store the formatted string.
    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x);

    // Rust has a print!() macro as well
    print!("Hello, world!"); // Without newline
    println!(); //  Newline

    print!("Hello, world!\n");
}
