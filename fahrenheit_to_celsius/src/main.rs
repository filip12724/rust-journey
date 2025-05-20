use std::io;

fn main() {
    let mut fahrenheit = String::new() ;
    

    println!("Please input fahrenheit");

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read the line");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) =>{
            eprintln!("error while converting number to string");
            return;
        },
    };

    let celsius= (fahrenheit - 32.0) * 5.0 / 9.0;


    println!("{:.1}°F = {:.1}°C", fahrenheit, celsius);
}
