fn main() {
    let x : u32 = 255;

    if x < 255 {
        println!("Hello, world!");
    } else {
        println!("Hello, universe!");
    }

    let zero = 0;

    if zero != 0 {
        println!("This was true");
    }else {
        println!("This was false");
    }

    let mut counter = 0;

    'counting_up: loop {
        println!("Count = {}", counter);
        let mut remaining = 10;

        loop {
            println!("Remaining = {}",remaining);

            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    };
    println!("End count = {}", counter);


    let mut number = 25;

    while number < 65 {
        println!("{}",number);
        number += 10;
    }
    println!("The final value of the number is {}", number);

    let arr = [5,6,7,8,9,12,15,16,17,189,1231];

    for a in arr {
        println!("The value of the current index is {}", a);
    }
     for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
