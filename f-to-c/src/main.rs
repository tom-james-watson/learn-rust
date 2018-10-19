use std::io;

fn get_farenheit() -> i32 {
    loop {
        let mut f = String::new();

        println!("Enter Farenheit value:");

        io::stdin()
            .read_line(&mut f)
            .expect("Failed to read line");

        let f: i32 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid value");
                continue
            }
        };

        break f
    }
}

fn main() {
    let f = get_farenheit();
    let c = (f - 32) * 5 / 9;

    println!("{}f is {}c", f, c)
}
