pub fn factors(n: u64) -> Vec<u64> {
  let mut primes: Vec<u32> = vec![];
  sieve_of_eratosthenes(&mut primes);
  let mut i = 0u32;
  let mut p = 2u64;
  let mut x = n;
  let mut pfs: Vec<u64> = vec![];
  while p * p <= x || x > 1 {
    while x % p == 0 {
      pfs.push(p);
      x = x / p;
    }
    i += 1;
    p = primes[i as usize] as u64;
  }
  pfs
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
