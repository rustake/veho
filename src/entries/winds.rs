use std::collections::HashMap;
use std::hash::Hash;

pub fn wind<KI, VI>(keys: KI, values: VI) -> HashMap<KI::Item, VI::Item> where
    KI: IntoIterator,
    VI: IntoIterator,
    KI::Item: Hash + Eq,
{
    keys.into_iter().zip(values).collect()
}

#[cfg(test)]
mod tests {
    use crate::vector::Mappers;

    use super::*;

    #[test]
    fn test_move_wind() {
        let keys = vec!["foo", "bar", "zen"];
        let values = vec!["1", "2", "3"];
        let hash_map = wind(keys, values);
        println!("{:?}", hash_map);
    }

    #[test]
    fn test_ref_wind() {
        let keys = vec!["foo", "bar", "zen"].mapper(|x| x.to_owned());
        let values = vec!["1", "2", "3"].mapper(|x| x.to_owned());
        let hash_map = wind(&keys, &values);
        println!("{:?}", hash_map);
    }

    #[test]
    fn test_move_wind_array() {
        let keys = ["foo", "bar", "zen"];
        let values = ["1", "2", "3"];
        let hash_map = wind(keys.iter(), values.iter());
        println!("{:?}", hash_map);
    }

    #[test]
    fn test_ref_wind_array() {
        let keys = ["foo", "bar", "zen"];
        let values = ["1", "2", "3"];
        let hash_map = wind(&keys, &values);
        println!("{:?}", hash_map);
    }
}
