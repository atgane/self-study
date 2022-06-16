fn main() {
    let mut s = String::from("hello world");

    let s1 = new_str(&s);
    println!("{}", s);
    println!("{}", s1);
    s.clear();
    println!("{}", s1);
}

fn new_str(s: &String) -> &str {
    &s[..]
}