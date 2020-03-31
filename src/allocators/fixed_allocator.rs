use std::marker::PhantomData;
use std::cmp::Ordering;

pub trait Indexes<ID>: Copy {
    fn index(&self) -> usize;
}

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

#[derive(Debug)]
pub struct Id<T> {
    pub(crate) index: usize,
    marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            index,
            marker: PhantomData,
        }
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index)
    }
}

impl<T> Eq for Id<T> {}

impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self::new(self.index)
    }
}

impl<T> Copy for Id<T> {}

impl<ID> Indexes<ID> for Id<ID> {
    fn index(&self) -> usize {
        self.index
    }
}

impl<'a, ID> Indexes<ID> for &'a Id<ID> {
    fn index(&self) -> usize {
        self.index
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
