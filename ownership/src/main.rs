fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        println!("{s}");
    } //s is no longer valid here, since the scope finished, it gets dropped

    // println!("{s}"); wouldn't work here, it's out of the scope
    // Normal strings are immutable, so we have to use the String type
        let s = String::from("hello");
        println!("{s}");

        let mut hello = String::from("hello");
        hello.push_str(", world!");
        println!("{hello}");

    // Normal strings are hardcoded, String types aren't

    {
        let mut peter = String::from("Hey peter!"); // Peter gets created
        peter.push_str(" hyd m8");
        println!("{peter}"); // We do stuff with peter
    } // Peter gets dropped, it gets deleted by Rust, since we're no longer going to use it.ç

    let s1 = String::from("hello"); //We create a string and a pointer to the string
    let s2 = s1; //We copy the pointer s1, since copying the string would be a waste of resources

    println!("{s2}");
    // println!("{s1}"); //This wouldn't work, because we deleted the old pointer, since we won't use it anymore
}
