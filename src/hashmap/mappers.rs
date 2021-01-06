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

pub fn mapper<K, V, MP, KT, VT, F>(kvp: MP, f: F) -> HashMap<KT, VT> where
    K: Hash + Eq,
    MP: IntoIterator<Item=(K, V)>,
    F: FnMut(K, V) -> (KT, VT),
    KT: Hash + Eq
{ kvp.mapper(f) }

pub fn iterate<K, V, MP, F>(kvp: MP, f: F) where
    K: Hash + Eq,
    MP: IntoIterator<Item=(K, V)>,
    F: FnMut(K, V)
{ kvp.iterate(f) }

pub fn mapper_key<K, V, MP, KT, F>(kvp: MP, f: F) -> HashMap<KT, V> where
    K: Hash + Eq,
    MP: IntoIterator<Item=(K, V)>,
    F: FnMut(K) -> KT,
    KT: Hash + Eq
{ kvp.mapper_key(f) }

pub fn iterate_key<K, V, MP, F>(kvp: MP, f: F) where
    K: Hash + Eq,
    MP: IntoIterator<Item=(K, V)>,
    F: FnMut(K)
{ kvp.iterate_key(f) }

pub fn mapper_value<K, V, MP, VT, F>(kvp: MP, f: F) -> HashMap<K, VT> where
    K: Hash + Eq,
    MP: IntoIterator<Item=(K, V)>,
    F: FnMut(V) -> VT,
{ kvp.mapper_value(f) }

pub fn iterate_value<K, V, MP, F>(kvp: MP, f: F) where
    K: Hash + Eq,
    MP: IntoIterator<Item=(K, V)>,
    F: FnMut(V)
{ kvp.iterate_value(f) }


#[cfg(test)]
mod hashmap_mappers_tests {
    use crate::hashmap::Init;
    use crate::hashmap::mappers::Mappers;

    #[test]
    fn test_mapper() {
        let alpha = vec![("foo", 1), ("bar", 2), ("zen", 3)].into_hashmap();
        println!("alpha = {:?}", alpha);
        let beta = (&alpha).mapper(|k, v| (k, v + 1));
        println!("beta = {:?}", beta);
        println!("alpha = {:?}", alpha);
    }

    #[test]
    fn test_iterate_mutate() {
        let mut dict = vec![("foo", 1), ("bar", 2), ("zen", 3)].into_hashmap();
        println!("{:?}", dict);
        (&mut dict).iterate(|_, v| { *v += 1; });
        println!("{:?}", dict);
    }

    #[test]
    fn test_mapper_value() {
        let dict = vec![("foo", 1), ("bar", 2), ("zen", 3)].into_hashmap();
        println!("{:?}", dict);
        let dict = dict.mapper_value(|v| v + 1);
        println!("{:?}", dict);
    }

    #[test]
    fn test_iterate_value() {
        let mut dict = vec![("foo", 1), ("bar", 2), ("zen", 3)].into_hashmap();
        println!("{:?}", dict);
        (&mut dict).iterate_value(|v| { *v += 1; });
        println!("{:?}", dict);
    }
}

#[cfg(test)]
mod hashmap_mappers_func_tests {
    use crate::hashmap::{Init, iterate, iterate_value, mapper, mapper_value};

    #[test]
    fn test_mapper() {
        let alpha = vec![("foo", 1), ("bar", 2), ("zen", 3)].into_hashmap();
        println!("alpha = {:?}", alpha);
        let beta = mapper(&alpha, |k, v| (k, v + 1));
        println!("beta = {:?}", beta);
        println!("alpha = {:?}", alpha);
    }

    #[test]
    fn test_iterate_mutate() {
        let mut dict = vec![("foo", 1), ("bar", 2), ("zen", 3)].into_hashmap();
        println!("{:?}", dict);
        iterate(&mut dict, |_, v| { *v += 1; });
        println!("{:?}", dict);
    }

    #[test]
    fn test_mapper_value() {
        let dict = vec![("foo", 1), ("bar", 2), ("zen", 3)].into_hashmap();
        println!("{:?}", dict);
        let dict = mapper_value(dict, |v| v + 1);
        println!("{:?}", dict);
    }

    #[test]
    fn test_iterate_value() {
        let mut dict = vec![("foo", 1), ("bar", 2), ("zen", 3)].into_hashmap();
        println!("{:?}", dict);
        iterate_value(&mut dict, |v| { *v += 1; });
        println!("{:?}", dict);
    }
}