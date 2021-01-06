use std::collections::HashMap;
use std::hash::Hash;

pub trait Init<K, V>: IntoIterator<Item=(K, V)> where
    K: Hash + Eq,
{
    fn into_hashmap(self) -> HashMap<K, V> where
        Self: Sized
    {
        self.into_iter().collect::<HashMap<K, V>>()
    }
}

impl<K, V, KVS: ?Sized> Init<K, V> for KVS where
    K: Hash + Eq,
    KVS: IntoIterator<Item=(K, V)> {}

pub fn init_to_hashmap<K, V, KVS>(kvs: KVS) -> HashMap<K, V> where
    K: Hash + Eq,
    KVS: IntoIterator<Item=(K, V)> {
    kvs.into_hashmap()
}


#[cfg(test)]
mod test {
    use crate::hashmap::cloner::Cloner;
    use crate::hashmap::init::init_to_hashmap;

    #[test]
    fn test() {
        let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])];
        let beta = init_to_hashmap(tuples.clone());
        let alpha = (tuples).into_hashmap();
        println!("{:?}", alpha);
        println!("{:?}", beta);
    }
}