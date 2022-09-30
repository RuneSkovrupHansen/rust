fn main() {
    let verses: [&str; 12] = [
        "a partridge in a pear tree",
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
        "Twelve drummers drumming",
    ];

    for i in 0..verses.len() {
        let day: usize = i + 1;
        println!("On the {day}. day of Christmas,\nmy true love sent to me");
        for j in (0..=i).rev() {
            let verse = if i != 0 && j == 0 {
                format!("And {}", verses[j])
            } else {
                capitalize_first_letter(verses[j])
            };

            let punctuation = if j == 0 { "." } else { "," };
            println!("{verse}{punctuation}");
        }
        println!()
    }
}

fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}
