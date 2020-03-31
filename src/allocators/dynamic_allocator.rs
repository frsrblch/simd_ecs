use std::num::NonZeroU32;
use crate::allocators::{Indexes, Id};
use crate::links::GenIds;
use crate::components::Comp1;
use crate::{Get1, Insert};
use std::cmp::Ordering;

#[derive(Debug, Default, Clone)]
pub struct DynamicAllocator<T> {
    pub(crate) gen: Comp1<T, Gen>,
    dead: Vec<Id<T>>,
    pub(crate) version: u64,
}

impl<T> DynamicAllocator<T> {
    pub fn create(&mut self) -> GenId<T> {
        if let Some(index) = self.dead.pop() {
            let gen = self.gen.get(index).copied().unwrap_or_default();
            GenId::new(index, gen)
        } else {
            let index = Id::new(self.gen.len());
            let gen = Gen::default();

            self.gen.insert(index, gen);

            GenId::new(index, gen)
        }
    }

    pub fn kill(&mut self, id: GenId<T>) {
        if self.is_valid(&id) {
            if let Some(gen) = self.gen.get_mut(id.index) {
                *gen = gen.next();
                self.dead.push(id.index);
            }
        }
        self.version += 1;
    }

    pub fn is_valid(&self, id: &GenId<T>) -> bool {
        self.gen
            .get(id.index)
            .map(|gen| *gen == id.gen)
            .unwrap_or(false)
    }

    pub fn is_alive(&self, id: Id<T>, gen: Gen) -> bool {
        self.gen
            .get(id)
            .map(|live| *live == gen)
            .unwrap_or(false)
    }

    pub fn validate<'a, ID2>(&'a self, ids: &'a mut GenIds<ID2, T>) -> Valid<'a, ID2, T> {
        ids.update(&self);
        Valid::new(&ids.ids.0)
    }
}

#[derive(Debug)]
pub struct GenId<T> {
    pub(crate) index: Id<T>,
    pub(crate) gen: Gen,
}

impl<T> GenId<T> {
    fn new(index: Id<T>, gen: Gen) -> Self {
        Self {
            index,
            gen,
        }
    }
}

impl<T> Clone for GenId<T> {
    fn clone(&self) -> Self {
        Self::new(self.index, self.gen)
    }
}

impl<T> Copy for GenId<T> {}

impl<T> PartialEq for GenId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index)
    }
}

impl<T> Eq for GenId<T> {}

impl<T> std::hash::Hash for GenId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.index, self.gen).hash(state)
    }
}

impl<T> PartialOrd for GenId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for GenId<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.index, self.gen).cmp(&(other.index, other.gen))
    }
}

impl<'a, T> Indexes<T> for GenId<T> {
    fn index(&self) -> usize {
        self.index.index
    }
}

impl<'a, T> Indexes<T> for &'a GenId<T> {
    fn index(&self) -> usize {
        self.index.index
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Gen(NonZeroU32);

impl Default for Gen {
    fn default() -> Self {
        Gen(NonZeroU32::new(1).unwrap())
    }
}

impl Gen {
    pub fn next(self) -> Self {
        let next = NonZeroU32::new(self.0.get() + 1).unwrap();
        Self(next)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Valid<'a, FROM, TO> {
    pub ids: &'a Comp1<FROM, Option<Id<TO>>>,
}

impl<'a, FROM, TO> Valid<'a, FROM, TO> {
    fn new(ids: &'a Comp1<FROM, Option<Id<TO>>>) -> Self {
        Valid {
            ids,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_two() {
        let mut a = DynamicAllocator::<()>::default();

        let id0 = a.create();
        assert_eq!(GenId::new(Id::new(0), Gen::default()), id0);

        let id1 = a.create();
        assert_eq!(GenId::new(Id::new(1), Gen::default()), id1);
    }

    #[test]
    fn create_destroy_create() {
        let mut a = DynamicAllocator::<()>::default();

        let id0 = a.create();
        a.kill(id0);
        let id1 = a.create();

        assert_eq!(GenId::new(Id::new(0), Gen::default().next()), id1);
    }
}
