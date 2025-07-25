use std::hash::{DefaultHasher, Hash, Hasher};
use std::marker::PhantomData;

/// Type-safe wrapper around a hash intended to avoid accidental mix-ups
pub struct ChangeDetector<T> {
    hash: u64,
    phantom: PhantomData<T>,
}

impl <T> ChangeDetector<T> where T : Hash {
    pub fn new() -> ChangeDetector<T> {
        ChangeDetector {
            hash: 0, // About a 1 in 18 quintillion chance of hash collision with initial value
            phantom: Default::default(),
        }
    }

    /// Check if detector has been used
    pub fn untouched(&self) -> bool {
        self.hash == 0
    }

    /// Access the inner hash
    pub fn hash(&self) -> u64 {
        self.hash
    }

    /// Returns Some when the value differs or is the first value
    pub fn detect<'a>(&mut self, value: &'a T) -> Option<&'a T> {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        let change = self.hash != hash;
        self.hash = hash;
        if change {
            Some(value)
        }
        else {
            None
        }
    }

    /// Useful to avoid cloning with non-copy types like String
    pub fn detect_owned(&mut self, value: T) -> Option<T> {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        let change = self.hash != hash;
        self.hash = hash;
        if change {
            Some(value)
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ChangeDetector;

    #[test]
    fn change_detect_works() {
        let mut change_detector = ChangeDetector::<usize>::new();
        assert_eq!(change_detector.detect(&1), Some(&1));
        assert_eq!(change_detector.detect(&1), None);
        assert_eq!(change_detector.detect(&2), Some(&2));
        assert_eq!(change_detector.detect(&1), Some(&1));
        assert_eq!(change_detector.detect(&1), None);
    }

    #[test]
    fn owned_change_detect_works() {
        let mut string_change_detector = ChangeDetector::<String>::new();
        let a1 = "A".to_string();
        assert_eq!(string_change_detector.detect_owned(a1), Some("A".to_string()));
        let b1 = "B".to_string();
        assert_eq!(string_change_detector.detect_owned(b1), Some("B".to_string()));
        let b2 = "B".to_string();
        assert_eq!(string_change_detector.detect_owned(b2), None);
    }

    #[test]
    fn example_works() {
        let mut writes = 0_usize;

        let mut write_to_file = |change_detector: &mut ChangeDetector<Vec<usize>>, nums: Vec<usize>| {
            if let Some(_nums) = change_detector.detect_owned(nums) {
                // WRITE DATA TO SOME FILE
                writes += 1;
            }
        };

        let mut change_detector = ChangeDetector::<Vec<usize>>::new();

        write_to_file(&mut change_detector, vec![1, 2, 3]);
        write_to_file(&mut change_detector, vec![1, 2, 3]);
        write_to_file(&mut change_detector, vec![1, 2, 3, 4]);
        write_to_file(&mut change_detector, vec![1, 2, 3, 4]);

        assert_eq!(writes, 2);
    }
}