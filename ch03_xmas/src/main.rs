fn main() {
    for n in 1..13 {
        verse(n);
    }
}

fn verse(v: usize) {
    let gifts = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming"
    ];

    let suffixes = ["st", "nd", "rd", "th"];
    let suffix_idx = if v < 4 { v - 1 } else { 3 };

    println!("On the {}{} day of Christmas, my true love gave to me:",
             v, suffixes[suffix_idx]);

    for n in (1..(v + 1)).rev() {
        let idx = n - 1;
        if n == 1 {
            println!("    {} {}",
                     if v == 1 { "a" } else { "and a" },
                     gifts[idx]);
        } else {
            println!("    {} {}", n, gifts[idx]);
        }
    }

    println!("");
}
