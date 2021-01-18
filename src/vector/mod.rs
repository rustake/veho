pub use inits::{init, iso};
pub use mappers::{indexed_iterate,
                  indexed_mapper, indexed_mutate, iterate,
                  mapper, Mappers, mutate};
pub use reduces::{mapreduce, reduce, Reduces};
pub use zippers::{quazipeach, quazipper, trizipeach,
                  trizipper, zipeach, zipper,
};

mod zippers;
mod mappers;
mod inits;
mod reduces;

