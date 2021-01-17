pub use inits::init;
pub use mappers::{indexed_iterate,
                  indexed_mapper, indexed_mutate, iterate,
                  mapper, Mappers, mutate};
pub use reduces::{map_reduce, Reduces, reduce};
pub use zippers::{quazipper, trizipper, zipper};


mod zippers;
mod mappers;
mod inits;
mod reduces;

