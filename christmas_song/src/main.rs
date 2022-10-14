fn main() {
    let ordinal = [
        "first", "second", "third",
        "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth"
    ];

    let lines = [
        "Twelve drummers drumming", "Eleven pipers piping", "Ten lords a-leaping",
        "Nine ladies dancing", "Eight maids a-milking", "Seven swans a-swimming",
        "Six geese a-laying", "Five golden rings", "Four calling birds",
        "Three french hens", "Two turtle doves, and", "A partridge in a pear tree",
    ];
    let mut count = 0;

    for number in ordinal {
        println!("On the {} day of Christmas, my true love sent to me", number);
        if count == 0 {
            println!("{}", lines[0]);
        }
        if count == 1 {
            println!("{}", lines[0]);
            println!("{}", lines[1]);
        }
        if count == 2 {
            println!("{}", lines[0]);
            println!("{}", lines[1]);
            println!("{}", lines[2]);
        }
        if count == 3 {
            println!("{}", lines[0]);
            println!("{}", lines[1]);
            println!("{}", lines[2]);
            println!("{}", lines[3]);
        }
        if count == 4 {
            println!("{}", lines[0]);
            println!("{}", lines[1]);
            println!("{}", lines[2]);
            println!("{}", lines[3]);
            println!("{}", lines[4]);
        }
        if count == 5 {
            println!("{}", lines[0]);
            println!("{}", lines[1]);
            println!("{}", lines[2]);
            println!("{}", lines[3]);
            println!("{}", lines[4]);
            println!("{}", lines[5]);
        }
        if count == 6 {
            println!("{}", lines[0]);
            println!("{}", lines[1]);
            println!("{}", lines[2]);
            println!("{}", lines[3]);
            println!("{}", lines[4]);
            println!("{}", lines[5]);
            println!("{}", lines[6]);
        }
        if count == 7 {
            println!("{}", lines[0]);
            println!("{}", lines[1]);
            println!("{}", lines[2]);
            println!("{}", lines[3]);
            println!("{}", lines[4]);
            println!("{}", lines[5]);
            println!("{}", lines[6]);
            println!("{}", lines[7]);
        }
        if count == 8 {
            println!("{}", lines[0]);
            println!("{}", lines[1]);
            println!("{}", lines[2]);
            println!("{}", lines[3]);
            println!("{}", lines[4]);
            println!("{}", lines[5]);
            println!("{}", lines[6]);
            println!("{}", lines[7]);
            println!("{}", lines[8]);
        }
        if count == 9 {
            println!("{}", lines[0]);
            println!("{}", lines[1]);
            println!("{}", lines[2]);
            println!("{}", lines[3]);
            println!("{}", lines[4]);
            println!("{}", lines[5]);
            println!("{}", lines[6]);
            println!("{}", lines[7]);
            println!("{}", lines[8]);
            println!("{}", lines[9]);
        }
        if count == 10 {
            println!("{}", lines[0]);
            println!("{}", lines[1]);
            println!("{}", lines[2]);
            println!("{}", lines[3]);
            println!("{}", lines[4]);
            println!("{}", lines[5]);
            println!("{}", lines[6]);
            println!("{}", lines[7]);
            println!("{}", lines[8]);
            println!("{}", lines[9]);
            println!("{}", lines[10]);
        }
        if count == 11 {
            println!("{}", lines[0]);
            println!("{}", lines[1]);
            println!("{}", lines[2]);
            println!("{}", lines[3]);
            println!("{}", lines[4]);
            println!("{}", lines[5]);
            println!("{}", lines[6]);
            println!("{}", lines[7]);
            println!("{}", lines[8]);
            println!("{}", lines[9]);
            println!("{}", lines[10]);
            println!("{}", lines[11]);
        }
        if count >= 12 {
            break;
        }

        count+=1;
        println!("");
    }
}
