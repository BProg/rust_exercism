pub fn is_armstrong_number(num: u32) -> bool {
  let exponent = num.digits().len() as u32;
  let sum_of_powers = num.digits().iter().fold(0, |sum, d| sum + d.pow(exponent));
  sum_of_powers == num
}

trait Digits {
  fn digits(&self) -> Vec<u32>;
}

impl Digits for u32 {
  fn digits(&self) -> Vec<u32> {
    self.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()
  }
}
