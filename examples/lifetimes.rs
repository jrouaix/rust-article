fn main() {
  let s = gimme_a_str("hé", "ho");
  println!("{}", s);
}

// fn gimme_a_str(first: &str, second: &str) -> &str {
//     second
// }

fn gimme_a_str<'a, 'b>(
  first: &'a str,
  second: &'b str,
) -> &'a str {
  second
}
