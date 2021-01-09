pub use inits::{clone_to_hashmap, into_hashmap, MoveInit, ref_to_hashmap, RefInit};
pub use mappers::{iterate, iterate_key, iterate_value,
                  mapper, mapper_key, mapper_value,
                  Mappers};

mod mappers;
mod inits;

