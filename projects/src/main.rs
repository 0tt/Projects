use std::io;

fn main() {
    println!("Hello, world!");
    println!("Pi(100): {}", pi(100));
    println!("terrible e(10000000000): {}", e(10000000000));
    println!("fib(10): {:?}", fib(10));
    println!("prime time of your life: {:?}", factors(17285));
    primes();
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

fn factors(mut num: u64) -> Vec<u64> {
    // Generate primes up to num
    let mut primes = vec![];
    'primer: for i in 2..num + 1 {
        for prime in &primes {
            if i % prime == 0 {
                continue 'primer;
            }
        }
        primes.push(i)
    }
    let mut factors = vec![];
    'x: loop {
        for prime in &primes {
            if num % prime == 0 {
                factors.push(*prime);
                // num /= prime; // Not implemented yet
                num = num / prime;
                if num == 1 {break 'x};
                break;
            }
        }
    }
    factors
}
fn primes() {
    let mut primes = vec![];
    loop {
        println!("More primes?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        if input.len() > 0 {
            let len = primes.len();
            let start = if len > 0 {primes[len - 1]} else {2};
            'primer: for i in start.. {
                for prime in &primes {
                    if i % prime == 0 {
                        continue 'primer;
                    }
                }
                primes.push(i);
                println!("New prime! {}", i);
                break;
            }
        } else {
            break;
        }
    }
}
