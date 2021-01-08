use std::collections::HashMap;
use std::hash::Hash;

pub trait Cloner<'a, K, V>: IntoIterator<Item=&'a (K, V)> where
    K: 'a + Hash + Eq + Clone,
    V: 'a + Clone,
{
    fn into_hashmap(self) -> HashMap<K, V> where
        Self: Sized
    {
        self.into_iter().map(|(k, v)| (k.clone(), v.clone())).collect::<HashMap<K, V>>()
    }
}

impl<'a, K, V, KVS: ?Sized> Cloner<'a, K, V> for KVS where
    K: 'a + Hash + Eq + Clone,
    V: 'a + Clone,
    KVS: IntoIterator<Item=&'a (K, V)> {}

pub fn clone_to_hashmap<'a, K, V, KVS>(kvs: KVS) -> HashMap<K, V> where
    K: 'a + Hash + Eq + Clone,
    V: 'a + Clone,
    KVS: IntoIterator<Item=&'a (K, V)>
{
    kvs.into_hashmap()
}

#[cfg(test)]
mod test {
    use crate::hashmap::cloner::Cloner;

    #[test]
    fn test_vec() {
        let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])];
        let beta = (&tuples).into_hashmap();
        let alpha = (tuples).into_hashmap();
        println!("{:?}", alpha);
        println!("{:?}", beta);
    }

    #[test]
    fn test_arr() {
        let tuples = [("one", [1]), ("two", [2]), ("three", [3])];
        let beta = (&tuples).into_hashmap();
        let alpha = (tuples).into_hashmap();
        println!("{:?}", alpha);
        println!("{:?}", beta);
    }
}