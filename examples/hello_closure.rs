fn main() {
    let name = String::from("closure");
    let closure = |s| format!("{} {}!", s, name);
    println!("{}", closure("Hello"));
    // println!("{}", closure(42));
    // expected `&str`, found integer
}

// cargo run --example hello_closure
//> Hello closure!
