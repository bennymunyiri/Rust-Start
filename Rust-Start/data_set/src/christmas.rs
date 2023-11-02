fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];

    println!("The Twelve Days of Christmas Lyrics\n");

    for day in 0..12 {
        println!("On the {} day of Christmas,", days[day]);
        println!("my true love sent to me:");

        for gift_day in (0..=day).rev() {
            if gift_day == 0 && day > 0 {
                print!("And ");
            }
            println!("{}", gifts[gift_day]);
        }

        println!();
    }
}
