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

    for number in ordinal {
        println!("On the {} day of Christmas, my true love sent to me", number);
    }

    for lyric in 1..12 {
        println!("{}", lines[lyric]);
    }
}
