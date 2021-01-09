pub use inits::ref_init::{clone_to_hashmap, RefInit};
pub use inits::move_init::{MoveInit, init_to_hashmap};
pub use mappers::{iterate, iterate_key, iterate_value,
                  mapper, mapper_key, mapper_value,
                  Mappers};

mod mappers;
mod inits;

