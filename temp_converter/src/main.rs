fn main() {
    let temperature : i32 = 0;
    let unit : char = 'c'; // c for celsius, f for fahrenheit

    if unit == 'c' {
        let temperature : i32 = temperature * (9/5) + 32;
        println!("The temperature in fahrenheit is {temperature}");
    } else if unit == 'f' {
        let temperature :i32 = (temperature - 32) * (5/9);
        println!("The temperature in celsius is {temperature}");
    }
}
