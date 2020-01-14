fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("the first word is: {}", &word);
    s.clear(); //

    // println!("the first word is: {}", &word); //error
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
