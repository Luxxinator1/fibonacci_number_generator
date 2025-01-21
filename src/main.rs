use std::io;

fn main() {
    loop {
        println!("Please input the nth Fibonacci number you want to know: ");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Please input a positive number!");

        let _n: u64 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("{n} is not a valid number!");
                continue;
            }
        };
    }
}
