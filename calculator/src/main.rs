fn main() {
    another_function(calculate(3, five()), 'f');
}

fn another_function(value: i32, name: char){
    println!("The value of the parameter is {value} and {name}");
}

fn five() -> i32{
    5;
    3
}
fn calculate(x: i32,y: i32)->i32{
    return x + y
}
