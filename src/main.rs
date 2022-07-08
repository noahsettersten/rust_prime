use std::env;

fn main() {
    let max = parse_argument();
    println!("Listing prime numbers up to {}", max);
    // let primes = brute_force(max);
    let primes = sieve_of_eratosthenes(max);

    for prime in primes.iter() {
        println!("{}", prime)
    }
    println!("")
}

fn parse_argument() -> usize {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return 1000;
    }

    match args[1].parse::<usize>() {
        Err(why) => panic!("{:?}", why),
        Ok(number) => number,
    }
}

fn sieve_of_eratosthenes(max: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::new();

    let numbers: Vec<usize> = (2..max).collect();
    let mut sieve: Vec<bool> = (0..max).map(|_x| false).collect();

    for num in numbers {
        let mut index = num;
        loop {
            index += num;
            if index >= max {
                break;
            }

            sieve[index] = true;
        }
    }

    for (i, s) in sieve.iter().enumerate() {
        if !s {
            primes.push(i);
        }
    }

    return primes;
}

fn brute_force(num: i64) -> Vec<i64> {
    let mut primes: Vec<i64> = Vec::new();

    for n in 2..=num {
        let mut prime = true;
        let root_of_n = (n as f64).sqrt().floor() as i64;

        for i in 2..=root_of_n {
            if n % i == 0 {
                prime = false;
            }
        }

        if prime {
            primes.push(n);
        }
    }

    return primes;
}
