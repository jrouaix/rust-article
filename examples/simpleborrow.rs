fn main() {
  let mut s = String::from("hello Rust!");

  // un emprunt modifiable dont le scope ..
  mutation(&mut s); // .. se termine ici

  // ou plusieurs emprunt non modifiable
  let a = &s;
  let b = &s;
  print(a, b);
}

fn mutation(s: &mut String) {
  s.push_str("!");
}

fn print(a: &String, b: &String) {
  dbg!(a, b);
}
