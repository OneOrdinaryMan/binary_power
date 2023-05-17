fn two_power(pow: usize) -> usize {
    if pow == 0 {
        1
    } else {
        2 * two_power(pow - 1)
    }
}
fn main() {
    let expr = two_power(4);
    println!("{}", expr);
}
