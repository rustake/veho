pub trait Unwinds<K, V>: IntoIterator<Item=(K, V)>
{
    fn unwind(self) -> (Vec<K>, Vec<V>) where
        Self: Sized
    { self.into_iter().unzip() }
}

impl<K, V, KVS: ?Sized> Unwinds<K, V> for KVS where
    KVS: IntoIterator<Item=(K, V)> {}

pub fn unwind<K, V, KVS>(kvs: KVS) -> (Vec<K>, Vec<V>) where
    KVS: IntoIterator<Item=(K, V)>
{ kvs.unwind() }


#[cfg(test)]
mod test {
    use crate::entries::IntoHashmap;

    use super::*;

    #[test]
    fn test_entries_unwind() {
        let tuples = vec![
            (1, 10.0),
            (2, 20.0),
            (3, 30.0),
        ];
        let (a, b) = tuples.unwind();
        println!(">> {:?}", a);
        println!(">> {:?}", b);
        // println!(">> {:?}", tuples);
    }

    #[test]
    fn test_entries_ref_unwind() {
        // let tuples = vec![
        //     (1, 10.0),
        //     (2, 20.0),
        //     (3, 30.0),
        // ];
        // let (a, b) = (&tuples).unwind();
        // println!(">> {:?}", a);
        // println!(">> {:?}", b);
        // println!(">> {:?}", tuples);
    }

    #[test]
    fn test_hashmap_ref_unwind() {
        let tuples = vec![
            (1, 10.0),
            (2, 20.0),
            (3, 30.0),
        ].into_hashmap();
        let (a, b) = (&tuples).unwind();
        println!(">> {:?}", a);
        println!(">> {:?}", b);
        println!(">> {:?}", tuples);
    }
}

#[cfg(test)]
mod prototype_tests {
    #[test]
    fn test_move_unwind() {
        let tuples = vec![
            (1, [10.0, 0.0]),
            (2, [20.0, 0.0]),
            (3, [30.0, 0.0]),
        ];
        let (a, b): (Vec<i32>, Vec<[f64; 2]>) = (&tuples).into_iter().map(|(x, y)| (x, y)).unzip();
        println!(">> {:?}", a);
        println!(">> {:?}", b);
    }
}

