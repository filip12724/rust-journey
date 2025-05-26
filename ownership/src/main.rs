fn main() {
    let mut s = String::from("Hello");

    s.push_str(" World!");

    println!("{}",s);
    let x = 5;
    let y = x;
    println!("{}\n{}",x,y);
    let mut s1 = String::from("Hellooooooooooooooo");

    calculate_len(&mut s1);
    println!("{s1}");

    let reference_to_nothing = dangle();
    println!("{reference_to_nothing}");
}
fn calculate_len(string2: &mut String){

    string2.push_str(", world");
}
fn dangle () -> String {
    let s = String::from("dangle");

    s// we return a reference to the String, s
} // Here, s goes out of scope, and is dropped, so its memory goes away.
  // Danger!