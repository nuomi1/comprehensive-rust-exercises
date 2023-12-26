/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut collatz: Vec<i32> = Vec::new();
    while n != 1 {
        collatz.push(n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    collatz.len() as u32
}

fn main() {
    let n = 3;
    println!("collatz_length(n) = {}", collatz_length(n));
}
