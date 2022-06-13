fn main() {
    let mut s = String::from("Hsdlsadlfsa");
    {
        let r = &mut s;
        r.push('a');
    }

    {
        let p = &mut s;
        p.push('a');
    }
    println!("{}", s);
}