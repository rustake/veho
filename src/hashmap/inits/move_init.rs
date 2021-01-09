use std::collections::HashMap;
use std::hash::Hash;

pub trait MoveInit<K, V>: IntoIterator<Item=(K, V)> where
    K: Hash + Eq,
{
    fn into_hashmap(self) -> HashMap<K, V> where
        Self: Sized
    {
        self.into_iter().collect::<HashMap<K, V>>()
    }
}

impl<K, V, KVS: ?Sized> MoveInit<K, V> for KVS where
    K: Hash + Eq,
    KVS: IntoIterator<Item=(K, V)> {}


#[cfg(test)]
mod test {
    use crate::hashmap::inits::into_hashmap;
    use crate::hashmap::RefInit;

    #[test]
    fn test_vec() {
        let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])];
        let beta = into_hashmap(tuples.clone());
        let alpha = (tuples).clone_to_hashmap();
        println!("{:?}", alpha);
        println!("{:?}", beta);
    }

    #[test]
    fn test_arr() {
        let tuples = [("one", [1]), ("two", [2]), ("three", [3])];
        // let beta = init_to_hashmap(tuples);
        let alpha = (tuples).clone_to_hashmap();
        println!("{:?}", alpha);
        // println!("{:?}", beta);
    }
}