fn main() {
    hello_world();
    i_like_params(10);
    several_moments_ago(10,'h');

    // let x = y = 10; This won't work
    let x = 10;
    let y = x; // This does
    println!("{y}");

    // This also does, since it does a expression, unlike let x = (let y = 4);, since `let` isn't an expression, it's a statement

    let a  = {
        let z = 3;
        z + 1
    };
    println!("{a}");

    let fifteen = number_fifteen_foot_lettuce();
    println!("{fifteen}");

    let x_and_y = y_plus_x(x, y);
    println!("X plus Y is {x_and_y}");
}

// Functions: fn (snake_case name)() { }
// They allow you to call it at any moment, and run a piece of it at any moment
fn hello_world() {
    println!("Hola Mundo!");
}

// You can set params by writing the name of the variable, AND THE TYPE
fn i_like_params(x: i32) {
    println!("x is equals to {x}");
}

// When handling several arguments, separate them with commas
fn several_moments_ago(time : i32, unit : char) {
    println!("That happened {time}{unit} ago")
}

// Functions can also return a value, the ones that return must have (input type, empty if no params) -> (exit type), and the value you want to return cannot have semicolons,
// since it would make it a statement
fn number_fifteen_foot_lettuce() -> i32 {
    15
}
fn y_plus_x(x : i32, y : i32) -> i32 {
    x + y
}