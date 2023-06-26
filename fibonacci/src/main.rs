use std::io;
fn main() {
    println!("Which fibonacci number would you like to calculate?");

    let n: u32 = loop {
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let _: u32 = match n.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    println!("F_{n} = {}", fib(n));

}

fn fib(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 || n == 2 {
        return 1;
    }

    let mut n_2: u64 = 0; // F_0
    let mut n_1: u64 = 1; // F_1
    let mut curr: u64 = n_1 + n_2; // F_2
    for _ in 3..=n {
        n_2 = n_1;
        n_1 = curr;
        curr = n_1 + n_2;
    }

    return curr;
}