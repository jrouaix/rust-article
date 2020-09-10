fn main() {
    let any = vec![21, 2, 12834, 2, 12, 12847, 3, 5, 7];
    let sum: u32 =
        any.into_iter().filter(|i| *i > 10).take(3).sum();
    dbg!(sum);
}
//> [examples\iterators.rs:7] sum = 12867
