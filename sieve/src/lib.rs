pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
  let mut primes = vec!();
  let mut is_prime: Vec<bool> = vec![];
  is_prime.resize((upper_bound + 1) as usize, true);
  let mut p = 2;
  while p * p <= upper_bound {
    if is_prime[p as usize] {
      let mut i = p * p;
      while i <= upper_bound {
        is_prime[i as usize] = false;
        i += p;
      }
    }
    p += 1
  }
  for (index, is_p) in is_prime.iter().enumerate() {
    if *is_p && index > 1 {
      primes.push(index as u64);
    }
  }
  primes
}
