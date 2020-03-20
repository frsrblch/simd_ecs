use std::marker::PhantomData;
use std::num::NonZeroU32;

#[derive(Debug, Default, Clone)]
pub struct DynamicAllocator<T> {
    gen: Vec<Gen>,
    dead: Vec<usize>,
    marker: PhantomData<T>,
}

impl<T> DynamicAllocator<T> {
    pub fn create(&mut self) -> GenId<T> {
        if let Some(index) = self.dead.pop() {
            let gen = self.gen.get(index).copied().unwrap_or_default();
            GenId::new(index, gen)
        } else {
            let index = self.gen.len();
            let gen = Gen::default();

            self.gen.push(gen);

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
    }

    pub fn is_valid(&self, id: &GenId<T>) -> bool {
        self.gen
            .get(id.index)
            .map(|gen| *gen == id.gen)
            .unwrap_or(false)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GenId<T> {
    index: usize,
    gen: Gen,
    marker: PhantomData<T>,
}

impl<T> GenId<T> {
    fn new(index: usize, gen: Gen) -> Self {
        Self {
            index,
            gen,
            marker: PhantomData,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_two() {
        let mut a = DynamicAllocator::<()>::default();

        let id0 = a.create();
        let id1 = a.create();

        assert_eq!(0, id0.index);
        assert_eq!(Gen::default(), id0.gen);
        assert_eq!(1, id1.index);
        assert_eq!(Gen::default(), id1.gen);
    }

    #[test]
    fn create_destroy_create() {
        let mut a = DynamicAllocator::<()>::default();

        let id0 = a.create();
        a.kill(id0);
        let id1 = a.create();

        assert_eq!(0, id0.index);
        assert_eq!(Gen::default(), id0.gen);
        assert_eq!(0, id1.index);
        assert_eq!(Gen::default().next(), id1.gen);
    }
}
