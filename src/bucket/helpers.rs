use kv::Key;

pub(crate) trait IndexOf<K: Key> {
    fn index_of(&self, key: K) -> Option<usize>;
}

pub(crate) struct WrappingIndexIterator {
    start: usize,
    length: usize,
    current: usize,
    first: bool,
}

impl WrappingIndexIterator {
    pub fn new(start: usize, length: usize) -> Self {
        assert!(start <= length);
        WrappingIndexIterator {
            start,
            length,
            current: start,
            first: true,
        }
    }
}

impl Iterator for WrappingIndexIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        // Check that we haven't returned to the beginning
        if self.current != self.start || self.first {
            self.first = false;

            // Check that we shouldn't wrap
            if self.current < self.length {
                let item = self.current;
                self.current += 1;
                Some(item)
            } else {
                self.current = 0;
                self.next()
            }
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_index_iterator() {
        let start = 0;
        let length = 5;
        let result = WrappingIndexIterator::new(start, length)
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(&result[..], &[0, 1, 2, 3, 4]);

        let start = 2;
        let length = 5;
        let result = WrappingIndexIterator::new(start, length)
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(&result[..], &[2, 3, 4, 0, 1]);

        let start = 0;
        let length = 1;
        let result = WrappingIndexIterator::new(start, length)
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(&result[..], &[0]);

        let start = 0;
        let length = 0;
        let result = WrappingIndexIterator::new(start, length)
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(&result[..], &[]);
    }

    #[test]
    #[should_panic]
    fn wrapping_index_iterator_invalid_input() {
        let start = 1;
        let length = 0;
        WrappingIndexIterator::new(start, length);
    }
}
