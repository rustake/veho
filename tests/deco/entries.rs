use std::fmt;

pub fn deco_entries<K, V, KVS>(entries: KVS, de: &str) -> String where
    K: fmt::Display,
    V: fmt::Display,
    KVS: IntoIterator<Item=(K, V)>
{
    let lines = entries.into_iter()
        .map(|(k, v)| format!("  {}{}{},", k, de, v))
        .collect::<Vec<String>>();
    // let lines: Vec<String> = mapper(entries, |k, v| format!("  {}{}{},", k, de, v));
    return format!("{{\n{}\n}}", lines.join("\n"));
}