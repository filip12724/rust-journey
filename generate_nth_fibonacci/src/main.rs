use std::io;

fn main() {

    println!("Enter which Fibonacci index you want:");

    let mut fibonacci = String::new();

    io::stdin()
        .read_line(&mut fibonacci)
        .expect("Failed to read the line");

    let n: u64 = match fibonacci.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please type a valid positive integer.");
            return;
        }
    };

    if n == 0 {
        println!("F₀ = 0");
        return;
    }
    if n == 1 {
        println!("F₁ = 1");
        return;
    }

    let mut prev : u64 = 1;
    let mut curr : u64 = 1;

    for _ in 3..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
     println!("Fₙ = {curr}");
}
