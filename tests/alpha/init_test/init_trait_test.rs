use std::hash::Hash;

pub trait Unwinds<K, V>: IntoIterator<Item=(K, V)> where
    Self: Sized,
    K: Hash + Eq
{
    fn move_unwind(self) -> (Vec<K>, Vec<V>) where
    {
        let some: (Vec<K>, Vec<V>) = self.into_iter().unzip();
        return some;
    }

    // fn ref_unwind<'a>(self) -> (Vec<&'a K>, Vec<&'a V>) where
    //     K: 'a,
    //     V: 'a,
    //     KVS: IntoIterator<Item=&'a (K, V)>
    // {
    //     self.into_iter().map(|&(ref a, ref b)| (a, b)).unzip()
    // }
    //
    // fn copy_unwind<'a>(self) -> (Vec<K>, Vec<V>) where
    //     K: 'a + Hash + Eq + Copy,
    //     V: 'a + Copy,
    //     KVS: IntoIterator<Item=&'a (K, V)>
    // {
    //     self.into_iter().map(|&(ref a, ref b)| (a, b)).unzip()
    // }
}

impl<K, V, I> Unwinds<K, V> for I where
    I: IntoIterator<Item=(K, V)>,
    K: Hash + Eq {}

#[cfg(test)]
mod test {
    use veho::entries::unwind;

    #[test]
    fn test_move_unwind() {
        let tuples = vec![
            (1, 10.0),
            (2, 20.0),
            (3, 30.0),
        ];
        let (a, b) = unwind(tuples);
        println!("{:?}", a);
        println!("{:?}", b);
        // println!("{:?}", tuples);
    }
}