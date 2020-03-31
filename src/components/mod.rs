use crate::allocators::{Id, GenId};
use crate::{Get1, Get2, Insert};
use std::marker::PhantomData;

pub use comp1::Comp1;
pub use comp2::Comp2;

mod comp1;
mod comp2;