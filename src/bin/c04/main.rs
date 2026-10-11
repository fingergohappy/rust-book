use std::io::Bytes;

use rand::rand_core::le;

fn main() {
    ref_test();
}

fn fn_a() {
    let s = String::from("hello");
    take_owner_ship(s);
    // 不能再使用了
    // println!("{s}");
    let x = 5;
    makes_copy(x);
}

fn take_owner_ship(str: String) {
    println!("{str}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn fn_b() {
    let s1 = give_owenship();
    let s2 = String::from("hello");
    let s3 = takes_and_give_back(s2);
}

fn give_owenship() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

fn ref_test() {
    let mut s = String::from("hello");
    let len = calculate_len(&s);
    println!("string :{s} len is {len}");
    change_str(&mut s);
    println!("string s changed : {s}");
}

fn calculate_len(s: &str) -> usize {
    s.len()
}

fn change_str(s: &mut String) {
    s.push_str(" world");
}

fn test_slice() {}

fn first_word_bad(s: String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_good(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
