use std::hash::Hash;

pub use move_unwinds::MoveUnwind;
pub use ref_unwinds::RefUnwind;

pub mod move_unwinds;
pub mod ref_unwinds;


pub fn move_unwind<K, V, KVS>(kvs: KVS) -> (Vec<K>, Vec<V>) where
    K: Hash + Eq,
    KVS: IntoIterator<Item=(K, V)>
{ kvs.move_unwind() }

pub fn ref_unwind<'a, K, V, KVS>(kvs: KVS) -> (Vec<&'a K>, Vec<&'a V>) where
    K: 'a + Hash + Eq,
    V: 'a,
    KVS: IntoIterator<Item=&'a (K, V)>
{ kvs.ref_unwind() }

pub fn clone_unwind<'a, K, V, KVS>(kvs: KVS) -> (Vec<K>, Vec<V>) where
    K: 'a + Hash + Eq + Clone,
    V: 'a + Clone,
    KVS: IntoIterator<Item=&'a (K, V)>
{ kvs.clone_unwind() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_unwind() {
        let tuples = [
            (1, 10.0),
            (2, 20.0),
            (3, 30.0),
        ];
        let (a, b) = clone_unwind(tuples.iter());
        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}", tuples);
    }

    #[test]
    fn test_move_unwind() {
        let tuples = vec![
            (1, 10.0),
            (2, 20.0),
            (3, 30.0),
        ];
        let (a, b) = move_unwind(tuples);
        println!("{:?}", a);
        println!("{:?}", b);
        // println!("{:?}", tuples);
    }

    #[test]
    fn test_ref_unwind() {
        let tuples = [
            (1, 10.0),
            (2, 20.0),
            (3, 30.0),
        ];
        let (a, b) = ref_unwind(&tuples);
        println!("{:?}", a);
        println!("{:?}", b);
    }
}




