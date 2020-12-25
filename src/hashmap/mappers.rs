use std::collections::HashMap;
use std::hash::Hash;

pub trait Mappers<K, V>: IntoIterator<Item=(K, V)> where K: Hash + Eq
{
    fn mapper<KT, VT, F>(self, mut f: F) -> HashMap<KT, VT> where
        Self: Sized,
        F: FnMut(K, V) -> (KT, VT),
        KT: Hash + Eq
    { self.into_iter().map(|(k, v)| f(k, v)).collect::<HashMap<KT, VT>>() }

    fn iterate<F>(self, mut f: F) where
        Self: Sized,
        F: FnMut(K, V)
    { self.into_iter().for_each(|(k, v)| f(k, v)) }

    fn mapper_key<KT, F>(self, mut f: F) -> HashMap<KT, V> where
        Self: Sized,
        F: FnMut(K) -> KT,
        KT: Hash + Eq
    { self.into_iter().map(|(k, v)| (f(k), v)).collect::<HashMap<KT, V>>() }

    fn iterate_key<F>(self, mut f: F) where
        Self: Sized,
        F: FnMut(K)
    { self.into_iter().for_each(|(k, _)| f(k)) }

    fn mapper_value<VT, F>(self, mut f: F) -> HashMap<K, VT> where
        Self: Sized,
        F: FnMut(V) -> VT,
    { self.into_iter().map(|(k, v)| (k, f(v))).collect::<HashMap<K, VT>>() }

    fn iterate_value<F>(self, mut f: F) where
        Self: Sized,
        F: FnMut(V)
    { self.into_iter().for_each(|(_, v)| f(v)) }
}

impl<K, V, KVS: ?Sized> Mappers<K, V> for KVS where
    K: Hash + Eq,
    KVS: IntoIterator<Item=(K, V)> {}


#[cfg(test)]
mod hashmap_mappers_tests {
    use crate::hashmap::init::tuples_to_hashmap;
    use crate::hashmap::mappers::{Mappers};

    #[test]
    fn test_mapper() {
        let dict = tuples_to_hashmap(vec![("foo", 1), ("bar", 2), ("zen", 3)]);
        println!("{:?}", dict);
        let dict = dict.mapper(|k, v| (k, v + 1));
        println!("{:?}", dict);
    }

    #[test]
    fn test_iterate_mutate() {
        let mut dict = tuples_to_hashmap(vec![("foo", 1), ("bar", 2), ("zen", 3)]);
        println!("{:?}", dict);
        (&mut dict).iterate(|_, v| { *v += 1; });
        println!("{:?}", dict);
    }

    #[test]
    fn test_mapper_value() {
        let dict = tuples_to_hashmap(vec![("foo", 1), ("bar", 2), ("zen", 3)]);
        println!("{:?}", dict);
        let dict = dict.mapper_value(|v| v + 1);
        println!("{:?}", dict);
    }

    #[test]
    fn test_iterate_value() {
        let mut dict = tuples_to_hashmap(vec![("foo", 1), ("bar", 2), ("zen", 3)]);
        println!("{:?}", dict);
        (&mut dict).iterate_value(|v| { *v += 1; });
        println!("{:?}", dict);
    }
}