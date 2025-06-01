fn main() {

    //------------------------OWNERSHIP---------------------------------//

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

    //----------------------SLICE TYPES------------------------------//

    let words = String::from("word1 word2 word3");
    first_word(&words);

    // let mut s = String::from("omg wtf amazing cool lmao");

    // let word = first_word(&s); 
    // println!("This is the first word of the given string: {word}");
    // s.clear();

    // let s = String::from("Hello world");
    let hello = &s[..5];
    let world = &s[6..11];
    println!("{hello} {world}");

    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear(); 
    
    // println!("the first word is: {word}");
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);
}
fn calculate_len(string2: &mut String){

    string2.push_str(", world");
}

fn dangle () -> String {
    let s = String::from("dangle");

    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}