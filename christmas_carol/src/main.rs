fn main() {
    let nums: [&str; 12] = [
        "First", "Second", "Third", "Fourth",
        "Fifth", "Sixth", "Seventh", "Eigth",
        "Ninth", "Tenth", "Eleventh", "Twelfth"
    ];
    
    let daily_gifts: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me", nums[day]);
        for gift in 0..=day {
            println!("{}", daily_gifts[gift])
        }
        println!();
    }

}
