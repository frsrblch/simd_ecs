use std::marker::PhantomData;
use crate::allocators::*;

pub trait Insert<ID, T> {
    fn insert(&mut self, id: ID, value: T);
}

#[derive(Debug, Default, Clone)]
pub struct Ids<FROM, TO> {
    indices: Vec<Id<TO>>,
    marker: PhantomData<FROM>,
}

impl<FROM, TO> Insert<Id<FROM>, Id<TO>> for Ids<FROM, TO> {
    fn insert(&mut self, id: Id<FROM>, value: Id<TO>) {
        self.insert_at_index(id.index, value);
    }
}

impl<FROM, TO> Insert<&Id<FROM>, Id<TO>> for Ids<FROM, TO> {
    fn insert(&mut self, id: &Id<FROM>, value: Id<TO>) {
        self.insert_at_index(id.index, value);
    }
}

// TODO Insert<Valid>

impl<FROM, TO> Ids<FROM, TO> {
    fn insert_at_index(&mut self, index: usize, value: Id<TO>) {
        match self.indices.len() {
            len if len == index => self.indices.push(value),
            len if len > index => {
                if let Some(v) = self.indices.get_mut(index) {
                    *v = value;
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GenIds<FROM, TO> {
    indices: Vec<usize>,
    gen: Vec<Gen>,
    marker: PhantomData<(FROM, TO)>,
}

// TODO Insert<Valid>
