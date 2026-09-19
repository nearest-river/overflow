

pub struct HeapPermutations<T> {
  data: Vec<T>,
  counters: Vec<usize>,
  i: usize,
  first: bool,
}

impl<T: Clone> HeapPermutations<T> {
  pub fn new(data:Vec<T>)-> Self {
    let n=data.len();

    Self {
      data,
      counters: vec![0; n],
      i: 0,
      first: true,
    }
  }
}

impl<T: Clone> Iterator for HeapPermutations<T> {
  type Item=Vec<T>;

  fn next(&mut self)-> Option<Self::Item> {
    if self.first {
      self.first=false;
      return Some(self.data.to_vec());
    }

    while self.i < self.data.len() {
      if !(self.counters[self.i] < self.i) {
        self.counters[self.i]=0;
        self.i+=1;
        continue;
      }

      if self.i % 2 == 0 {
        self.data.swap(0, self.i);
      } else {
        self.data.swap(self.counters[self.i], self.i);
      }

      self.counters[self.i] += 1;
      self.i = 0;

      return Some(self.data.to_vec());
    }

    None
  }
}

pub fn permutations<T,F: FnMut(&[T])>(mut data: Vec<T>,mut f: F) {
  fn heap<T,F: FnMut(&[T])>(data: &mut [T],n: usize,f: &mut F) {
    if n == 1 {
      f(data);
      return;
    }

    heap(data, n - 1, f);

    for i in 0..n - 1 {
      if n % 2 == 0 {
        data.swap(i, n - 1);
      } else {
        data.swap(0, n - 1);
      }

      heap(data, n - 1, f);
    }
  }

  if !data.is_empty() {
    let len=data.len();
    heap(&mut data,len,&mut f);
  }
}



