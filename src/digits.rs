
pub struct Digits {
  n: u32,
  is_zero: bool,
}

impl Digits {
  pub const fn new(n: u32)-> Self {
    Self {
      n,
      is_zero: n==0,
    }
  }
}

impl Iterator for Digits {
  type Item=u32;

  #[inline]
  fn next(&mut self)-> Option<Self::Item> {
    if self.n==0 {
      let is_zero=self.is_zero;
      self.is_zero=false;

      return is_zero.then_some(0);
    }

    let digit=self.n%10;
    self.n/=10;

    Some(digit)
  }
}

