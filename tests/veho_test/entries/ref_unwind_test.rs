use veho::entries::{IntoHashmap, unwind};

fn ref_unwind<'a, K, V, KVS>(it: KVS) -> (Vec<&'a K>, Vec<&'a V>) where
    KVS: IntoIterator<Item=(&'a K, &'a V)>,
{ it.into_iter().map(|(k, v)| (k, v)).unzip() }

#[test]
fn test() {
    let tuples = vec![
        (1, 10.0),
        (2, 20.0),
        (3, 30.0),
    ].into_hashmap();
    let (a, b) = unwind(&tuples);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", tuples);
}