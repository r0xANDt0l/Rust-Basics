fn main() {
    //Classic if
    // lets you run code if a condition is true

    let number: i32 = -10;

    if number >= 10 {
        println!("Number is more or equals to 10! The value of it is {number}");
        //You can put else to run some code if said condition is false
    } else {
        println!("The number is less than 10!, The value of it is {number}");
    }
    // If there isn't an else, the program will just skip it

    let numero : i32 = 5;

    // You always need to provide a boolean

    if numero != 15 {
        println!("wait a minute, the number isn't 15!")
    }

    // You can also handle several conditions with else if

    let divisor = 6;

    if divisor % 4 == 0 {
        println!("Divisible by 4");
    } else if divisor % 3 == 0 {
        println!("Divisible by 3");
    } else if divisor % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("Not divisible by 2, 3 or 4");
    }

    //Rust will stop reading the else ifs when the first true match

    // We can also bound a value to a variable if a case is true, both values must be the same type

    let condition = false;
    let numbar = if condition { 7 } else { 27 };

    println!("Numbar is {numbar}");

}
