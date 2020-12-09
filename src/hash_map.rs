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
    pub fn new() -> Self {
        // seven buckets for the start
        HashMap {
            buckets: Vec::new(),
        }
    }

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

    pub fn is_empty(&self) -> bool {
        self.buckets.len() == 0
    }

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
        assert_eq!(sut.get_bucket(&String::from("Rust is nice")), sut.get_bucket(&String::from("Rust is nice")));
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
