fn main() {
    let mut s = String::from("Hello");

    s.push_str(" World!");

    println!("{}",s);
    let x = 5;
    let y = x;
    println!("{}\n{}",x,y);
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{s1}, world!");
}
