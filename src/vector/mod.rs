mod zippers;
mod mappers;
mod inits;

pub use inits::{init};
pub use mappers::{Mappers,
                  mapper, iterate, mutate,
                  indexed_mapper, indexed_iterate, indexed_mutate};
pub use zippers::{zipper, trizipper, quazipper};