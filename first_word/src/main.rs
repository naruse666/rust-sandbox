fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hoge hoge");
    let word = first_word(&s[..]);
    println!("first word is {}", word);

    let my_str_literal = "hello world";
    let word = first_word(&my_str_literal[..]);
    println!("first word is {}", word);

    let word = first_word(my_str_literal);
    println!("first word is {}", word);
}
