pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    sieve_of_eratosthenes(&mut primes);
    primes[n as usize] as u32
}

fn sieve_of_eratosthenes(primes: &mut Vec<u32>) {
    const N: usize = 1_000_000;
    let mut is_prime: [bool; N] = [true; N];
    let mut p = 2;
    while p * p < N {
        if is_prime[p] {
            let mut i = p * p;
            while i < N {
                is_prime[i] = false;
                i += p;
            }
        }
        p += 1
    }
    for (index, is_p) in is_prime.iter().enumerate() {
        if *is_p && index > 1 {
            primes.push(index as u32);
        }
    }
}
