fn main() {
    let name = String::from("closure");
    let closure = |s| format!("{} {}!", s, name);
    println!("{}", closure("Hello"));
}

// cargo run --example hello_closure
//> Hello closure!
