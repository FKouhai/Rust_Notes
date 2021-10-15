fn main() {
    let mut s = String::from("hello World");
    let word = first_word(&s);
    //    s.clear();
    println!("The first word is {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //We check for anything before any whitespace
            return &s[0..i];
        }
    }
    &s[..]
}
