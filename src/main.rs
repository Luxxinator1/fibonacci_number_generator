use std::io;

fn main() {
    loop {
        println!("Please input the nth Fibonacci number you want to know: ");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Please input a positive number!");

        let n: u64 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("{n} is not a valid number!");
                continue;
            }
        };

        let result = fib(n);

        fn fib(n: u64) -> u64 {
            if n <= 0 {
                return 0;
            } else if n == 1 {
                return 1;
            } else {
                return fib(n - 1) + fib(n - 2);
            }
        }

        println!("The {n}th Fibonacci number is: {result}");

        break;
    }
}
