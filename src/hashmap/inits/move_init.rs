use std::collections::HashMap;
use std::hash::Hash;

// fn max<T>(vec: Vec<T>) -> Option<T> where T: Num {
//     match vec.iter().next() {
//         None => { None }
//         Some(mut max) => {
//             for x in vec.into_iter() { if x > max { max = x }; }
//             Some(max)
//         }
//     }
// }

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

pub fn init_to_hashmap<K, V, KVS>(kvs: KVS) -> HashMap<K, V> where
    K: Hash + Eq,
    KVS: IntoIterator<Item=(K, V)> {
    kvs.into_hashmap()
}


#[cfg(test)]
mod test {
    use crate::hashmap::inits::ref_init::RefInit;
    use crate::hashmap::inits::move_init::init_to_hashmap;

    #[test]
    fn test_vec() {
        let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])];
        let beta = init_to_hashmap(tuples.clone());
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