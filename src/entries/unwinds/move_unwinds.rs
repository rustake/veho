pub trait MoveUnwind<K, V>: IntoIterator<Item=(K, V)>
{
    fn move_unwind(self) -> (Vec<K>, Vec<V>) where
        Self: Sized
    { self.into_iter().unzip() }
}

impl<K, V, KVS: ?Sized> MoveUnwind<K, V> for KVS where
    KVS: IntoIterator<Item=(K, V)> {}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_unwind() {
        let tuples = vec![
            (1, 10.0),
            (2, 20.0),
            (3, 30.0),
        ];
        let (a, b) = tuples.move_unwind();
        println!("{:?}", a);
        println!("{:?}", b);
        // println!("{:?}", tuples);
    }
}