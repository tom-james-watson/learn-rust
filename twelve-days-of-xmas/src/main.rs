fn name_for_day(day: i8) -> String {
    match day {
        0 => String::from("first"),
        1 => String::from("second"),
        2 => String::from("thrid"),
        3 => String::from("fourth"),
        4 => String::from("fifth"),
        5 => String::from("sixth"),
        6 => String::from("seventh"),
        7 => String::from("eigth"),
        8 => String::from("ninth"),
        9 => String::from("tenth"),
        10 => String::from("eleventh"),
        11 => String::from("twelfth"),
        _ => panic!("Invalid day")
    }
}

fn gift_for_day(day: i8) -> String {
    match day {
        0 => String::from("a partridge in a pear tree"),
        1 => String::from("two turtle doves"),
        2 => String::from("three French hens"),
        3 => String::from("four calling birds"),
        4 => String::from("five golden rings"),
        5 => String::from("six geese a-layin'"),
        6 => String::from("seven swans a-swimmin'"),
        7 => String::from("eight maids a-milkin'"),
        8 => String::from("nine lords a-leapin'"),
        9 => String::from("ten ladies dancin'"),
        10 => String::from("eleven pipers pipin'"),
        11 => String::from("twelve drummers drummin'"),
        _ => panic!("Invalid day")
    }
}

fn main() {
    for i in 0..12 {
        let day = name_for_day(i);

        let mut gifts = String::new();

        for j in (0..i+1).rev() {
            let join = match j {
                _ if j == i => "\n",
                0 => "\nand ",
                _ => ",\n"
            };
            gifts = format!("{}{}{}", gifts, join, gift_for_day(j));
        }

        println!("On the {} of Christmas, my true love gave to me {}\n", day, gifts);
    }
}
