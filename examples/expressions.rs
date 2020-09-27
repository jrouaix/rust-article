fn main() {
    let result = pure_expression_function(1, 2);
    dbg!(result);
}

fn pure_expression_function(a: i8, b: i8) -> i8 {
    match a {
        0 => b + 1,
        1..=2 => pure_expression_function(3, 5),
        3 => {
            if b == 12 {
                b + a
            } else {
                a - b
            }
        }
        _ => panic!(),
    }
}
