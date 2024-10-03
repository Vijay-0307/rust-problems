fn main() {
    let mut s1 = String::from("vijay");
    do_something(&mut s1);
    println!("number uis {}", s1);
}

fn do_something(s2: &mut String) {
    s2.push_str(" sivaraman");
    println!("{}", s2); // s2 owns the value
}