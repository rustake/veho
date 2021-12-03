use std::collections::HashMap;
use std::hash::Hash;

pub trait IntoHashmap<'a, K, V>: IntoIterator<Item=&'a (K, V)> where
    K: 'a + Hash + Eq,
    V: 'a,
{
    fn into_hashmap(self) -> HashMap<&'a K, &'a V> where
        Self: Sized
    { self.into_iter().map(|&(ref k, ref v)| (k, v)).collect::<HashMap<&K, &V>>() }

    fn clone_to_hashmap(self) -> HashMap<K, V> where
        K: Clone,
        V: Clone,
        Self: Sized
    { self.into_iter().map(|(k, v)| (k.clone(), v.clone())).collect::<HashMap<K, V>>() }
}

impl<'a, K, V, KVS: ?Sized> IntoHashmap<'a, K, V> for KVS where
    K: 'a + Hash + Eq,
    V: 'a,
    KVS: IntoIterator<Item=&'a (K, V)> {}

pub fn into_hashmap<'a, K, V, KVS>(kvs: KVS) -> HashMap<&'a K, &'a V> where
    K: 'a + Hash + Eq,
    V: 'a,
    KVS: IntoIterator<Item=&'a (K, V)>
{ kvs.into_hashmap() }

pub fn clone_to_hashmap<'a, K, V, KVS>(kvs: KVS) -> HashMap<K, V> where
    K: 'a + Hash + Eq + Clone,
    V: 'a + Clone,
    KVS: IntoIterator<Item=&'a (K, V)>
{ kvs.clone_to_hashmap() }


#[cfg(test)]
mod test_clone_to_hashmap {
    use super::*;

    #[test]
    fn test_vec() {
        let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])];
        let beta = (&tuples).clone_to_hashmap();
        let alpha = (tuples).clone_to_hashmap();
        println!(">> [alpha] {:?}", alpha);
        println!(">> [beta] {:?}", beta);
        println!(">> [tuples] {:?}", tuples);
    }

    #[test]
    fn test_arr() {
        let tuples = [("one", [1]), ("two", [2]), ("three", [3])];
        let beta = (&tuples).clone_to_hashmap();
        let alpha = (tuples).clone_to_hashmap();
        println!(">> {:?}", alpha);
        println!(">> {:?}", beta);
    }
}

#[cfg(test)]
mod test_ref_to_hashmap {
    use super::*;

    #[test]
    fn test_vec() {
        let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])];
        let beta = (&tuples).into_hashmap();
        let alpha = (&tuples).clone_to_hashmap();
        println!(">> {:?}", alpha);
        println!(">> {:?}", beta);
        println!(">> {:?}", tuples);
    }

    #[test]
    fn test_arr() {
        let tuples = [("one", [1]), ("two", [2]), ("three", [3])];
        let beta = (&tuples).into_hashmap();
        let alpha = (&tuples).clone_to_hashmap();
        println!(">> {:?}", alpha);
        println!(">> {:?}", beta);
    }
}

