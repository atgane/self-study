fn main() {
let mut s1 = String::from("hello");
let mut s2 = s1.clone();

prints(&mut s1);
println!("s1 = {}, s2 = {}", &s1, &s2);
}

fn prints(s: &mut String) {
    println!("{}", s);
    s.pop();
}