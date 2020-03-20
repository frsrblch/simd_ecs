use std::marker::PhantomData;

#[derive(Debug, Default, Clone)]
pub struct FixedAllocator<T> {
    next_index: usize,
    marker: PhantomData<T>,
}

impl<T> FixedAllocator<T> {
    pub fn create(&mut self) -> Id<T> {
        let id = Id::new(self.next_index);
        self.next_index += 1;
        id
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Id<T> {
    pub(crate) index: usize,
    marker: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(index: usize) -> Self {
        Self {
            index,
            marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_two() {
        let mut a = FixedAllocator::<()>::default();

        let id0 = a.create();
        let id1 = a.create();

        assert_eq!(0, id0.index);
        assert_eq!(1, id1.index);
    }
}
