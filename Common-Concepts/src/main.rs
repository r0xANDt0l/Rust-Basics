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

    /*
    Int types

    There are several types, 8 bit , 16 bit, 32 bit, 64 bit and 128 bits. there's also one that depends on the architecture of your CPU (32 bit, 64 bit, etc) that is represented with `size`.
    These can be signed (Can be positive or negative) or unsigned (only positive). Signed has a range of -(2^n-1) -> (2^n-1) - 1, and unsigned 0 -> (2^n-1) - 1
    The main reason to use the size one is when you're indexing a collection

    By default, Rust makes all integers i32

    You can use _ between numbers to make it easier to read
    1000000 -> 1_000_000
    */

    let signed_8bit : i8 = -120;
    let unsigned_8bit : u8 = 250;

    println!("{signed_8bit}, {unsigned_8bit}");

    /*
    Float types:

    All floats are signed, and they can be 32 bits or 64 bits (f32 or f64 respectively)
    f32 has a bit more precision
    f64 is roughly the same speed

     */
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("x: {x}, y: {y}");

    // Operations

    let suma = 5 + 10;

    let resta :f32 = 19.9 - 4.3;

    let multiplicacion = 7 * 27;

    let cociente : f32 = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    let resto = 43%5;

    println!("Suma: {suma}, Resta: {resta}, multiplicacion: {multiplicacion}, cociente: {cociente}, cociente sin decimales (esta como i32): {floored}, resto: {resto}");


    //Bools

    // Used mainly in If's

    // Literally "True" or "False"

    let t = true; // Without specifying

    let f : bool = false; //explicit type annotation

    //Chars

    // Just a single UTF-8 (unicode) character

    let c = 'z';

    let z : char = 'ℤ'; //explicit type annotation

    let hmm = '🤔';
}
