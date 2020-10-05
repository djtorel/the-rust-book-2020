fn main() {
    let s1 = String::from("Hello World!");

    println!("The first word is: {}", first_word(&s1));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
