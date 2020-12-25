mod alias;
mod inits;
mod mappers;
mod zippers;

pub use alias::Matrix;
pub use inits::{init, iso};
pub use mappers::{Mappers, Mutaters,
                  mapper, iterate, mutate,
                  indexed_mapper, indexed_iterate, indexed_mutate};
pub use zippers::{zipper, trizipper, quazipper};


