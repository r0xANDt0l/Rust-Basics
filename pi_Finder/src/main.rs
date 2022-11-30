fn main() {
    let mut k : f64 = 1.0;
    let mut s : f64 = 0.0;

    for i in 1..1000000000000_i64 {
        if i % 2 == 0 {
            s += 4.0/k;
        } else {
            s -= 4.0/k;
        }
        k += 2.0;
    }
    println!("{s}");
}