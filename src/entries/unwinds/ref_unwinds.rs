pub trait RefUnwind<'a, K, V>: IntoIterator<Item=&'a (K, V)> where
    K: 'a,
    V: 'a,
{
    fn ref_unwind(self) -> (Vec<&'a K>, Vec<&'a V>) where
        Self: Sized
    { self.into_iter().map(|&(ref k, ref v)| (k, v)).unzip() }

    fn clone_unwind(self) -> (Vec<K>, Vec<V>) where
        K: Clone,
        V: Clone,
        Self: Sized
    { self.into_iter().map(|(k, v)| (k.clone(), v.clone())).unzip() }
}

impl<'a, K, V, KVS: ?Sized> RefUnwind<'a, K, V> for KVS where
    K: 'a,
    V: 'a,
    KVS: IntoIterator<Item=&'a (K, V)> {}


#[cfg(test)]
mod test_clone_unwind {
    use super::*;

    #[test]
    fn test_vec() {
        let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])];
        let alpha = (tuples).clone_unwind();
        let beta = (&tuples).clone_unwind();
        println!("{:?}", alpha);
        println!("{:?}", beta);
    }

    #[test]
    fn test_arr() {
        let tuples = [("one", [1]), ("two", [2]), ("three", [3])];
        let alpha = (tuples).clone_unwind();
        let beta = (&tuples).clone_unwind();
        println!("{:?}", alpha);
        println!("{:?}", beta);
    }
}

#[cfg(test)]
mod test_ref_unwind {
    use super::*;

    #[test]
    fn test_vec() {
        let tuples = vec![("one", [1]), ("two", [2]), ("three", [3])];
        let beta = (&tuples).ref_unwind();
        let alpha = (&tuples).clone_unwind();
        println!("{:?}", alpha);
        println!("{:?}", beta);
        println!("{:?}", tuples);
    }

    #[test]
    fn test_arr() {
        let tuples = [("one", [1]), ("two", [2]), ("three", [3])];
        let beta = (&tuples).ref_unwind();
        let alpha = (&tuples).clone_unwind();
        println!("{:?}", alpha);
        println!("{:?}", beta);
    }
}