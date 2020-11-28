fn main() {
  println!("{}", hello("World"));
  assert_eq!(hello("Programmez!"), "Hello Programmez!!");
}

/// # Examples
///
/// ```
/// # use crate::*;
/// let result = hello("Programmez!");
/// assert_eq!(result, "Hello Programmez!!")
/// ```
fn hello(name: &str) -> String {
  format!("Hello {}!", name)
}
