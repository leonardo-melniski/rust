pub fn is_divide_by(x: u32, dividers: &Vec<u32>) -> bool {
    if dividers.len() < 1 {
        return false;
    }
    return dividers
        .into_iter()
        .any(|it| x % it == 0);
}

pub fn next_prime(primes: &Vec<u32>) -> u32 {
    if primes.len() <= 1 {
        return primes.len() as u32 + 2;
    }
    let mut x = primes[primes.len() - 1];
    loop {
        x += 2;
        if !is_divide_by(x, &primes) {
            return x;
        }
    }
}

pub fn nth(n: u32) -> u32 {
    let mut v: Vec<u32> = vec![2, 3];
    loop {
        if (v.len() as u32) > n {
            return v[n as usize];
        };
        
        let prime = next_prime(&v);
        if v.len() % 1_000 == 0 {

            println!("{} {}", v.len(), prime);
        }
        v.push(prime);
    }
}
