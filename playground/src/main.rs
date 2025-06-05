fn main() {
    let sentences = ["This is my sentence", "what this?", "watafak man", "", "  "];
    for sentence in sentences {
        println!(
            "First word of '{sentence}' is: '{}'",
            first_word(sentence)
        );
        println!(
            "Second word of '{sentence}' is: '{}'",
            second_word(sentence)
        );
        println!("");
    }
}

#[allow(dead_code)]
fn first_word_slower(s: &str) -> String {
    let mut word = String::new();
    for c in s.chars() {
        if c == ' ' { break } else { word.push(c) }
    }
    word
}

#[allow(dead_code)]
fn first_word(s: &str) -> &str {
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

#[allow(dead_code)]
fn second_word(s: &str) -> &str {
    let mut start: usize = 0;
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            if start == 0 {
                start = i + 1
            } else {
                return &s[start..i];
            }
        }
    }
    &s[start..]
}
