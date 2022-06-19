fn main() {
    let mut count: u32 = 1;
    let mut num: u64 = 1;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);

    while count < 10 {
        num += 2;
        count += 1;
        if vector_is_prime(num, &primes) {
            primes.push(num);
        }
    }
    println!("{:?}", primes);
}

fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool {
    for &i in p {
        if num > i && num % i == 0 {
            return false;
        }
    }

    true
}
