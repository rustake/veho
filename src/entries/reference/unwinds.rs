pub trait Unwinds<'a, K, V>: IntoIterator<Item=&'a (K, V)> where
    K: 'a,
    V: 'a,
{
    fn unwind(self) -> (Vec<&'a K>, Vec<&'a V>) where
        Self: Sized
    { self.into_iter().map(|&(ref k, ref v)| (k, v)).unzip() }

    fn clone_unwind(self) -> (Vec<K>, Vec<V>) where
        K: Clone,
        V: Clone,
        Self: Sized
    { self.into_iter().map(|(k, v)| (k.clone(), v.clone())).unzip() }
}

impl<'a, K, V, KVS: ?Sized> Unwinds<'a, K, V> for KVS where
    K: 'a,
    V: 'a,
    KVS: IntoIterator<Item=&'a (K, V)> {}

pub fn unwind<'a, K, V, KVS>(kvs: KVS) -> (Vec<&'a K>, Vec<&'a V>) where
    K: 'a,
    V: 'a,
    KVS: IntoIterator<Item=&'a (K, V)>
{ kvs.unwind() }

pub fn clone_unwind<'a, K, V, KVS>(kvs: KVS) -> (Vec<K>, Vec<V>) where
    K: 'a + Clone,
    V: 'a + Clone,
    KVS: IntoIterator<Item=&'a (K, V)>
{ kvs.clone_unwind() }


#[cfg(test)]
mod test_unwind {
    use super::*;

    #[test]
    fn test_entries_unwind() {
        let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])];
        let alpha = (&tuples).unwind();
        let beta = (&tuples).unwind();
        println!("{:?}", alpha);
        println!("{:?}", beta);
    }

    #[test]
    fn test_hashmap_unwind() {
        // let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])].into_hashmap();
        // let alpha = (&tuples).unwind();
        // let beta = (&tuples).unwind();
        // println!("{:?}", alpha);
        // println!("{:?}", beta);
    }
}

#[cfg(test)]
mod test_clone_unwind {
    use super::*;

    #[test]
    fn test_vec() {
        let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])];
        let beta = (&tuples).clone_unwind();
        let alpha = (&tuples).clone_unwind();
        println!("{:?}", alpha);
        println!("{:?}", beta);
        println!("{:?}", tuples);
    }

    #[test]
    fn test_arr() {
        let tuples = [("one", [1]), ("two", [2]), ("three", [3])];
        let beta = (&tuples).clone_unwind();
        let alpha = (&tuples).clone_unwind();
        println!("{:?}", alpha);
        println!("{:?}", beta);
    }
}