use std::collections::HashMap;
use std::hash::Hash;

pub fn tuples_to_hashmap<K: Eq + Hash, V>(tuples: Vec<(K, V)>) -> HashMap<K, V> {
    return tuples.into_iter().collect::<HashMap<K, V>>();
    // return HashMap::<K, V>::from_iter(tuples.into_iter());
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::hashmap::init::tuples_to_hashmap;

    #[test]
    fn test() {
        // let vec = vec![(1, 2), (3, 4)];
        // let hash_map = from_vec(&vec);
        // println!("{:?}", hash_map);
        let tuples: Vec<(&str, i32)> = vec![("one", 1), ("two", 2), ("three", 3)];
        // let m: HashMap<&str, i32> = tuples.into_iter().collect();
        let m: HashMap<&str, i32> = tuples_to_hashmap(tuples);
        // let m = HashMap::<&str, i32>::from_iter(tuples.into_iter());
        println!("{:?}", m);
    }
}