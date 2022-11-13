fn main() {
    let arr = vec![100, 105, 110, 99, 0];
    let result: Vec<_> = arr.iter().filter(|&&n| n > 100).collect();
    println!("{:?}", result);
}
