mod mappers;
mod zippers;
mod mapper_trait;
mod enumerate;

pub use mappers::{mapper, iterate, mutate};
pub use mapper_trait::{Mapper};
pub use zippers::{zipper, tri_zipper, quo_zipper};