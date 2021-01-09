use std::collections::HashMap;
use std::hash::Hash;

pub use move_init::MoveInit;
pub use ref_init::RefInit;

pub mod move_init;
pub mod ref_init;

pub fn into_hashmap<K, V, KVS>(kvs: KVS) -> HashMap<K, V> where
    K: Hash + Eq,
    KVS: IntoIterator<Item=(K, V)>
{ kvs.into_hashmap() }

pub fn ref_to_hashmap<'a, K, V, KVS>(kvs: KVS) -> HashMap<&'a K, &'a V> where
    K: 'a + Hash + Eq,
    V: 'a,
    KVS: IntoIterator<Item=&'a (K, V)>
{ kvs.ref_to_hashmap() }

pub fn clone_to_hashmap<'a, K, V, KVS>(kvs: KVS) -> HashMap<K, V> where
    K: 'a + Hash + Eq + Clone,
    V: 'a + Clone,
    KVS: IntoIterator<Item=&'a (K, V)>
{ kvs.clone_to_hashmap() }
