fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    let len = calculate_length(&s1);
    println!("The length of '{}' is {} after chaged.", s1, len);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // slice

    let s = String::from("hello");
    let len = s.len();
    println!("value is {}", &s[0..2]);
    println!("value is {}", &s[..2]);
    println!("value is {}", &s[..]);
    println!("value is {}", &s[0..len]);
    println!("value is {}", &s[3..]);
    println!("value is {}", &s[3..len]);

    println!("value is {}", first_word("wordddddddd ddd"))
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn calculate_length(s: &String) -> usize {
    s.len()
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
