use std::collections::HashMap;
use std::hash::Hash;

pub trait IntoHashmap<K, V>: IntoIterator<Item=(K, V)> where
    K: Hash + Eq,
{
    fn into_hashmap(self) -> HashMap<K, V> where
        Self: Sized
    {
        self.into_iter().collect::<HashMap<K, V>>()
    }
}

impl<K, V, KVS: ?Sized> IntoHashmap<K, V> for KVS where
    K: Hash + Eq,
    KVS: IntoIterator<Item=(K, V)> {}

pub fn into_hashmap<K, V, KVS>(kvs: KVS) -> HashMap<K, V> where
    K: Hash + Eq,
    KVS: IntoIterator<Item=(K, V)>
{ kvs.into_hashmap() }


#[cfg(test)]
mod test {
    use crate::entries::into_hashmaps::into_hashmap;
    use crate::entries::reference::IntoHashmap;

    #[test]
    fn test_vec() {
        let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])];
        let beta = into_hashmap(tuples.clone());
        let alpha = (tuples).clone_to_hashmap();
        println!(">> {:?}", alpha);
        println!(">> {:?}", beta);
    }

    #[test]
    fn test_arr() {
        let tuples = [("one", [1]), ("two", [2]), ("three", [3])];
        // let beta = init_to_hashmap(tuples);
        let alpha = (tuples).clone_to_hashmap();
        println!(">> {:?}", alpha);
        // println!(">> {:?}", beta);
    }
}