use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

pub struct HashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
}

type Bucket<K, V> = Option<Vec<(K, V)>>;

const INITIAL_SIZE: usize = 7;

impl<K, V> HashMap<K, V>
where
    K: Hash + Ord,
    V: Ord,
{
    /// Creates an instance
    ///
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::hash_map::HashMap;
    /// let map = HashMap::<(), ()>::new();
    ///
    /// assert!(map.is_empty());
    /// ```
    pub fn new() -> Self {
        // seven buckets for the start
        HashMap {
            buckets: Vec::new(),
        }
    }

    /// Resizes the map. This could have multiple reasons:
    /// - the map is not yet initialize -> create the a list of buckets which
    ///   contains INITIAL_SIZE elements (currently seven).
    /// - the list of buckets doesn't contain enough buckets and therefore the 
    ///   size needs to be doubled. In that case the current content of the 
    ///   buckets should be redistributed.
    fn resize(&mut self) {
        let mut target_size: usize = INITIAL_SIZE;
        if self.buckets.len() != 0 {
            target_size = self.buckets.len() * 2;
        }

        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| None));

        //TODO: Copying existing values
        let _ = mem::replace(&mut self.buckets, new_buckets);
    }

    /// Checks if a map is currently empty.
    ///
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::hash_map::HashMap;
    /// let map = HashMap::<(), ()>::new();
    ///
    /// assert!(map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.buckets.len() == 0
    }

    /// Returns the bucket for a certain key. As the key needs to implement
    /// the Hash trait, we're able to hash it with the DefaultHasher.
    /// Afterwards the bucket is retrieved by calculating the remainder of
    /// the hash with the number of buckets.
    fn get_bucket(&self, key: &K) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let hash = hasher.finish();

        // TODO check if the cast is necessary
        return Some((hash % self.buckets.capacity() as u64) as usize);
    }

    /// Inserts a tuple into the HashMap.
    ///
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::hash_map::HashMap;
    /// let mut map = HashMap::new();
    /// map.insert(1, "Hello World");
    ///
    /// assert!(!map.is_empty());
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        //TODO:  or too small
        if self.is_empty() {
            self.resize()
        }

        if let Some(bucket) = self.get_bucket(&key) {
            match &mut self.buckets[bucket] {
                Some(vector) => vector.push((key, value)),
                None => {
                    self.buckets[bucket] = Some(Vec::with_capacity(10));
                    self.buckets[bucket].as_mut().unwrap().push((key, value));
                }
            }
        }
    }

    /// Returns a value for a given key.
    ///
    /// # Example
    /// ```rust
    /// use data_structure_with_colin::hash_map::HashMap;
    /// let mut map = HashMap::new();
    /// map.insert(42, 607);
    ///
    /// assert_eq!(map.get(42), Some(&607));
    /// ```
    pub fn get(&mut self, key: K) -> Option<&V> {
        if self.is_empty() {
            return None;
        }

        if let Some(bucket) = self.get_bucket(&key) {
            return match &self.buckets[bucket] {
                Some(vec) => vec
                    .into_iter()
                    .find(|k| k.0 == key)
                    .map_or(None, |tuple| Some(&tuple.1)),
                None => None,
            };
        }
        return None;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert() {
        let mut sut = HashMap::new();
        sut.insert(1, "Colin ist dof");

        assert!(!sut.is_empty());
    }

    #[test]
    fn test_get_bucket() {
        let sut = HashMap::<i64, ()>::new();
        assert_eq!(sut.get_bucket(&1), sut.get_bucket(&1));
        assert_eq!(sut.get_bucket(&11201020120), sut.get_bucket(&11201020120));

        let sut = HashMap::<String, ()>::new();
        assert_eq!(
            sut.get_bucket(&String::from("Rust is nice")),
            sut.get_bucket(&String::from("Rust is nice"))
        );
    }

    #[test]
    fn test_get() {
        let mut sut = HashMap::new();
        sut.insert(1, "Colin ist dof");

        assert_eq!(sut.get(1), Some(&"Colin ist dof"));
        assert!(!sut.is_empty());
    }

    #[test]
    fn test_get_int() {
        let mut sut = HashMap::new();
        sut.insert(1, 42);

        assert_eq!(sut.get(1), Some(&42));
        assert_eq!(sut.get(2), None);
        assert!(!sut.is_empty());
    }
}
