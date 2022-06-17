fn main() {
    let s = [1, 2, 3];
    
    let a = f1(s[0]);

    match a {
        None => println!("even number"),
        Some(i) => println!("odd number {}", i)
    }

    if let Some(i) = a {
        println!("odd number {}", i);
    }
    else {
        println!("even number");
    }
}

fn f1(a: i32) -> Option<i32> {
    if a % 2 == 0 {
        return None
    }
    return Some(1)
}