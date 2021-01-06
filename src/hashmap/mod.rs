pub use cloner::{clone_to_hashmap, Cloner};
pub use init::{Init, init_to_hashmap};
pub use mappers::{iterate, iterate_key, iterate_value,
                  mapper, mapper_key, mapper_value,
                  Mappers};

mod init;
mod mappers;
mod cloner;

