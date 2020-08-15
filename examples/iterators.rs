fn main() {
    let any = vec![
        1, 21, 2, 12834, 2, 2, 8, 12, 0, 12847, 1, 2, 1, 2, 7,
    ];
    let sum: u32 =
        any.into_iter().filter(|i| *i > 10).take(3).sum();
    dbg!(sum);
}
//> [examples\iterators.rs:7] sum = 12867
