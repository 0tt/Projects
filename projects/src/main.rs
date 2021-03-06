extern crate curl;
extern crate serde_json;

use std::io;
use std::str::FromStr;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Mul;
use std::env;

use curl::easy::Easy;
use serde_json::Value;

fn main() {
    println!("Hello, world!");
    println!("Pi(100): {}", pi(100));
    println!("terrible e(10000000000): {}", e(10000000000));
    println!("fib(10): {:?}", fib(10));
    println!("prime time of your life: {:?}", factors(17285));
    primes();
    tile();
    // mortgage();
    change();
    assert_eq!(decimal_to_binary_lazy(49), "0b110001");
    assert_eq!(decimal_to_binary(49), "0b110001");
    assert_eq!(decimal_to_binary(-987), "-0b1111011011");
    assert_eq!(binary_to_decimal("0b110001"), 49);
    assert_eq!(binary_to_decimal("-0b1111011011"), -987);
    calculator();
    zipf();
    cities();
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

fn read_type<T>() -> T  where T: Default + FromStr, <T as FromStr>::Err: Display {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or_else(|e| {
        println!("{}", e);
        T::default()
    })
}

fn tile() {
    println!("Enter the width, height, and per-square-unit cost:");
    let w = read_type::<u64>();
    let h = read_type::<u64>();
    let cost = read_type::<f64>();
    println!("Your total cost is: {}", (w * h) as f64 * cost);
}

#[allow(dead_code)]
fn mortgage() {
    println!("Enter the interest percentage, number of monthly payments, and ammount borrowed.");
    let r = read_type::<f64>() / 100.0;
    let n = read_type::<u64>() as f64;
    let p = read_type::<f64>();
    let c = (r * p) / (1.0 - (1.0 + r).powf(-n));
    println!("{} {} {} {}", c, r, n, p)
}

fn change() {
    println!("Enter the cost of a product and the amount of money given:");
    let cost = read_type::<f64>();
    let tender = read_type::<f64>();
    let mut change = ((tender - cost) * 100.0) as i64; // in terms of pennies (not tenths of pennies!)
    if change < 0 {
        println!("You didn't pay enough money!");
        return
    }
    let dollars = change / 100;
    change %= 100;
    let quarters = change / 25;
    change %= 25;
    let dimes = change / 10;
    change %= 10;
    let nickels = change /  5;
    change %= 5;
    let pennies = change;

    println!("Your total change is {} dollars, {} quarters, {} dimes, {} nickels, and {} pennies.", dollars, quarters, dimes, nickels, pennies);
}

fn decimal_to_binary_lazy(num: i64) -> String {
    let sign = if num.is_negative() { "-" } else { "" }.to_string();
    let num = num.abs() as u64;
    let mut working = num;
    (0..(num/2))
    .map(|x| 2u64.pow(x as u32))
    .filter(|&x| x <= num) // replace filter with take_while?
    .rev()
    .map(|x| {
        if working >= x {
            working -= x;
            "1"
        } else {
            "0"
        }
    })
    .fold(sign + "0b", |sum, x| sum + &x)
}
fn decimal_to_binary(num: i64) -> String {
    let sign = if num.is_negative() { "-" } else { "" }.to_string();
    let mut result = String::new();
    let mut working = num.abs() as u64;

    while working > 0 {
        result = (working % 2).to_string() + &result;
        working /= 2;
    }

    sign + "0b" + &result
}
fn binary_to_decimal(bin: &str) -> i64 {
    let mut result = 0i64;
    let mut sign = 1;
    let len = bin.len();
    for (n, c) in bin.chars().enumerate() {
        if n < 2 {
            if c == '-' { sign = -1 };
        } else if c == '1' {
            result += 2u64.pow((len - n) as u32 - 1) as i64;
        }
    }
    result * sign
}

fn calculator() {
    let mut answer: f64;
    loop {
        println!("Continue calculating?");
        let a = read_type::<String>();
        if a == "" { break };
        println!("Enter first operand:");
        let op1 = read_type::<f64>();
        println!("Enter operation:");
        let op = read_type::<String>();
        println!("Enter second operand:");
        let op2 = read_type::<f64>();

        answer = match &*op {
            "*" => op1 * op2,
            "/" => op1 / op2,
            "+" => op1 + op2,
            "-" => op1 - op1,
            _ => {
                println!("Invalid!");
                break;
            },
        };
        println!("{} {} {} = {}", op1, op, op2, answer);
    }
}

// negative numbers are for chumps
fn gcd(a: u64, b: u64) -> u64 {
    if b != 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

// prioritize not overflowing over speed
fn lcm(x: u64, y: u64) -> (u64, u64, u64) {
    let mut a: u64;
    let b: u64;
    if x > y {
        a = x;
        b = y;
    } else {
        a = y;
        b = x;
    }
    loop {
        a += a;
        if a % b == 0 {
            return (a, a / x, a / y);
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Fraction {
    numerator: u64,
    denominator: u64,
    sign: bool,
}
impl Fraction {
    fn reduce(&self) -> Fraction {
        let cd = gcd(self.numerator, self.denominator);
        if cd != 1 {
            Fraction {
                numerator: self.numerator / cd,
                denominator: self.denominator / cd,
                sign: self.sign,
            }
        } else {
            *self
        }
    }
    fn new(numerator: i64, denominator: i64) -> Fraction {
        Fraction {
            numerator: numerator.abs() as u64,
            denominator: denominator.abs() as u64,
            sign: !(numerator.is_negative() | denominator.is_negative()),
        }
    }
}
impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, _rhs: Fraction) -> Fraction {
        Fraction {
            numerator: self.numerator * _rhs.numerator,
            denominator: self.denominator * _rhs.denominator,
            sign: !(self.sign | _rhs.sign),
        }
    }
}
impl Add for Fraction {
    type Output = Fraction;

    fn add(self, _rhs: Fraction) -> Fraction {
        let l = self;
        let r = _rhs;
        let (cm, lc, rc) = lcm(l.denominator, r.denominator);
        Fraction {
            numerator: l.numerator * lc + r.numerator * rc,
            denominator: cm,
            sign: if self.sign | _rhs.sign {
                if self.sign {
                    l.numerator * lc > r.numerator * rc
                } else {
                    r.numerator * rc > l.numerator * lc
                }
            } else {
                self.sign
            },
        }.reduce()
    }
}
#[allow(dead_code)]
fn zipf_fraction() {
    println!("How many iterations do you want to generate zipf?");
    let iter = read_type::<u64>();
    let mut total = Fraction::new(0, 1);
    let mut last = Fraction::new(1, 1);
    for i in 1 .. iter+1 {
        last = last * Fraction::new(1, i as i64);
        total = total + last;
    }
    println!("{:?}", total);
}
fn zipf() {
    println!("How many iterations do you want to generate zipf?");
    let iter = read_type::<u64>();
    // let mut total = Fraction::new(0, 1);
    // let mut last = Fraction::new(1, 1);
    // for i in 1 .. iter+1 {
    //     last = last * Fraction::new(1, i as i64);
    //     total = total + last;
    // }
    // println!("{:?}", total);
    let mut total = 0f64;
    let mut frac = 1f64;
    for i in 1 .. iter + 1 {
        frac = frac / (i as f64);
        total = total + frac;
    }
    println!("{}", total);
}

#[derive(Debug, Default)]
struct Location {
    address: String,
    coordinates: (f64, f64),
}
fn find_city(name: &str) -> Option<Location> {
    let key = &match env::var("MAPS_API_KEY") {
        Ok(val) => val,
        Err(e) => {
            println!("bad key: {}", e);
            e.to_string()
        },
    };
    let mut easy = Easy::new();
    easy.url(&format!("https://maps.googleapis.com/maps/api/geocode/json?key={}&address={}", key, name.replace(" ", "+"))).unwrap();
    //let mut response;
    let mut response = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            // this is a vector of bytes; use from_utf8
            response.extend_from_slice(data);
            println!("chunk received!");
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap(); // program halts while above code collects chunks of data
    }
    let response_code = easy.response_code().unwrap();
    if response_code == 200 {
        let response = std::str::from_utf8(&response).unwrap(); // closure might outlive current function! (it doesn't)
        // println!("response {}", response);
        let data: Value = serde_json::from_str(response).unwrap();
        let obj = data.as_object().unwrap();
        let results = obj.get("results").unwrap().as_array().unwrap();
        if results.len() > 0 {
            let result = results[0].as_object().unwrap();
            let address = result.get("formatted_address").unwrap().as_str().unwrap();
            let location = result.get("geometry").unwrap().as_object().unwrap().get("location").unwrap().as_object().unwrap();
            let coordinates = (location.get("lat").unwrap().as_f64().unwrap(), location.get("lng").unwrap().as_f64().unwrap());
            // println!("Address: {}", address);
            // println!("Coordinates: {:#?}", coordinates);
            Some(Location {
                address: address.to_string(),
                coordinates: coordinates,
            })
        } else {
            println!("no results!");
            None
        }
    } else {
        println!("bad response code! {}", response_code);
        None
    }
}
fn spherical_law_of_cosines(c1: (f64, f64), c2: (f64, f64)) -> f64 {
    let (lat1, lng1) = c1;
    let (lat2, lng2) = c2;
    // d = acos( sin φ1 ⋅ sin φ2 + cos φ1 ⋅ cos φ2 ⋅ cos Δλ ) ⋅ R
    let phi1 = lat1.to_radians();
    let phi2 = lat2.to_radians();
    let deltalambda = (lng2 - lng1).to_radians();
    let r = 6371e3;

    (phi1.sin() * phi2.sin() + phi1.cos() * phi2.cos() * deltalambda.cos()).acos() * r
}
fn cities() {
    let mut city1 = Location::default();
    let city2; // man I don't even know, compiler yells at me
    let mut stage = true;
    loop {
        println!("Enter a city:");
        let input = read_type::<String>();
        if input.len() == 0 && stage {
            println!("Skipping.");
            return;
        }
        match find_city(&input) {
            Some(c) => {
                println!("Is this your intended location? (y/n) {}", c.address);
                match &*read_type::<String>() {
                    "y" | "Y" => {
                        if stage {
                            println!("City 1 selected.");
                            city1 = c;
                            stage = false;
                        } else {
                            println!("City 2 selected.");
                            city2 = c;
                            break;
                        }
                    },
                    _ => {},
                }
            },
            None => println!("City not found! Try again."),
        }
    }
    println!("City 1: {:?}\nCity 2: {:?}", city1, city2);
    println!("Distance between the two: {:.*}km (±22km)", 0, spherical_law_of_cosines(city1.coordinates, city2.coordinates)/1000f64);
}
