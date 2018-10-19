use std::io;

fn get_n() -> u128 {
    loop {
        let mut n = String::new();

        println!("Enter sequence number to find:");

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: u128 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue
            }
        };

        break n
    }
}

fn fib(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let n = get_n();
    let value = fib(n);

    println!("{} fibonacci number is {}", n, value);
}
