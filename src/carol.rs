pub fn christmas() {
    let wn: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelfth",
    ];
    let lns: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..12 {
        println!(
            "\nOn the {} day of Christmas, my true love sent to me",
            wn[i]
        );
        for j in (0..(i + 1)).rev() {
            println!("{}", lns[j]);
        }
    }
}
