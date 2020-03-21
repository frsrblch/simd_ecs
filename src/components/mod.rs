use crate::allocators::Id;
use crate::links::Insert;
use std::marker::PhantomData;

pub use comp1::Comp1;
pub use comp2::Comp2;

mod comp1;
mod comp2;