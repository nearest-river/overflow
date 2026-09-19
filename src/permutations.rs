pub struct HeapPermutations<T> {
    data: Vec<T>,
    counters: Vec<usize>,
    i: usize,
    first: bool,
}

impl<T: Clone> HeapPermutations<T> {
    pub fn new(data: Vec<T>) -> Self {
        let n = data.len();

        Self {
            data,
            counters: vec![0; n],
            i: 0,
            first: true,
        }
    }
}

impl<T: Clone> Iterator for HeapPermutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.data.clone());
        }

        while self.i < self.data.len() {
            if self.counters[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.data.swap(0, self.i);
                } else {
                    self.data.swap(self.counters[self.i], self.i);
                }

                self.counters[self.i] += 1;
                self.i = 0;

                return Some(self.data.clone());
            } else {
                self.counters[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}
