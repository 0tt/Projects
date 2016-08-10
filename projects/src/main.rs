fn main() {
    println!("Hello, world!");
    println!("Pi(100): {}", pi(100));
    println!("terrible e(10000000000): {}", e(10000000000));
    println!("fib(10): {:?}", fib(10))
}

fn pi(n: u64) -> f64 {
    // Nilakantha Series
    let result: f64 = (2..)
        .filter(|&x| x % 2 == 0) // Go by evens
        .take(n as usize) // iterate to specified
        .map(|x| x as f64) // convert to float
        .fold(3.0f64, |sum, x| {
            let d = 4.0 / (x * (x + 1.0) * (x + 2.0));
            sum + if x % 4.0 == 0.0 {-d} else {d}
        });
    result
}

fn e(n: u64) -> f64 {
    let n = n as f64;
    (1.0 + 1.0/n).powf(n)
}

fn fib(n: u64) -> Vec<u64> {
    let mut result = vec![1u64, 1u64];
    for _ in 2..n { // Should probably be 3..n + 1
        let len = result.len();
        let sum = result[len - 1] + result[len - 2];
        result.push(sum);
    }
    result
}
