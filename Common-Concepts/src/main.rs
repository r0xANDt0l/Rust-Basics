fn main() {
    // Variables
    let mut x = 5; // Naming convention for variables -> all_lowercase_with_underscores
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

    println!("The value of y in the outside scope is {}", y);

    // this will work

    let spaces = "   ";
    let spaces = spaces.len();

    println!("there are {} spaces", spaces);

    // this wont
    /*
    let mut spaces = "   ";
    spaces = spaces.len();

    println!("{spaces}");

    It won't work because we're trying to change the variable type (String to int)
    */

    // ========== //
    // Data types //
    // ========== //

    //Since `guess` can be any data type, we need to specify what kind is the number, since the rust compiler needs more info to know what we want it for

    let guess : u8 = "42"
        .parse()
        .expect("Not a number!");
    //this could be a single line, but it's  h a r d  t o  r e a d


    println!("{guess}");

}
