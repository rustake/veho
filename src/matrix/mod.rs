pub use alias::Matrix;
pub use inits::{init, iso};
pub use mappers::{indexed_iterate,
                  indexed_mapper, iterate,
                  mapper, Mappers};
pub use mutaters::{indexed_mutate, mutate, Mutaters};
pub use zippers::{quazipper, trizipper, zipper};

mod alias;
mod inits;
mod mappers;
mod zippers;
mod mutaters;

