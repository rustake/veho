use std::collections::HashMap;

type Dict = HashMap<String, Vec<String>>;

#[cfg(test)]
mod hashmap_test {
    use veho::hashmap::Mappers;

    use crate::deco::deco_entries;
    use crate::hashmap::hashmap_test::Dict;

    #[test]
    fn reference_test() {
        let mut dict = Dict::new();
        dict.insert("Gesualdo".to_string(),
                    vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
        dict.insert("Caravaggio".to_string(),
                    vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
        dict.insert("Cellini".to_string(),
                    vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);
        let dict2 = (&dict).mapper(|k, v| { (k, format!("{:?}", v)) });
        let text = deco_entries(&dict2, ": ");
        println!(">> dict = {:}", text);
        show(&dict);
        println!(">> dict = {:?}", dict);
    }

    fn show(table: &Dict) {
        for (artist, works) in table {
            println!(">> [{}]", artist);
            for work in works {
                println!("     {}", work);
            }
        }
    }
}
