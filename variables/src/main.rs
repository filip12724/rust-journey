fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of the const is {THREE_HOURS_IN_SECONDS}");
    let x = 5;
    
    let x = x + 2;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x in the outer scope is {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces : {spaces}");                  

    let _small_number : u8 = 255;

    let tup = (21, "Filip", "18-02-2004");

    let (age, name, birth_date) = tup;

    println!("The name is: {name}, age: {age}, birth date: {birth_date}");

    let _first_array = [1,2,3,4,5];

}
