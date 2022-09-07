fn main() {
    // Variables
    let mut x = 5; // Naming convention for variables -> all_like_this_with_underscores
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Naming convention for consts -> ALL_CAPS_WITH_UNDERSCORES

    println!("Three hours is {} seconds", THREE_HOURS_IN_SECONDS);

    // Shadowing

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The falue of y in the inner scope (Between the curly brackets) is {}" , y);
    }

    println!("The value of y in the outside scope is {}", y)
}
