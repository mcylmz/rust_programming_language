fn main() {
    // Variables and mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("constant is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let y = 5;

    let y = y + 1;
    
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "    "; // string type
    println!("spaces is: {spaces}");
    let spaces = spaces.len(); // number type
    println!("spaces is: {spaces}");
}
