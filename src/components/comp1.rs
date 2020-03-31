use super::*;
use crate::allocators::{Indexes, Valid};
// use crate::allocators::{Gen, DynamicAllocator};

#[derive(Debug, Clone)]
pub struct Comp1<ID, T> {
    pub values: Vec<T>,
    marker: PhantomData<ID>,
}

impl<ID, T> Default for Comp1<ID, T> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            marker: PhantomData,
        }
    }
}

impl<ID, T, I: Indexes<ID>> Insert<I, T> for Comp1<ID, T> {
    fn insert(&mut self, id: I, value: T) {
        self.insert_index(id.index(), value);
    }
}

impl<ID, T> Get1<Id<ID>, T> for Comp1<ID, T> {
    fn get(&self, id: Id<ID>) -> Option<&T> {
        self.get_index(id.index)
    }

    fn get_mut(&mut self, id: Id<ID>) -> Option<&mut T> {
        self.get_mut_index(id.index)
    }
}

impl<ID, T> Get1<&Id<ID>, T> for Comp1<ID, T> {
    fn get(&self, id: &Id<ID>) -> Option<&T> {
        self.get_index(id.index)
    }

    fn get_mut(&mut self, id: &Id<ID>) -> Option<&mut T> {
        self.get_mut_index(id.index)
    }
}

impl<ID, T> Get1<Option<Id<ID>>, T> for Comp1<ID, T> {
    fn get(&self, id: Option<Id<ID>>) -> Option<&T> {
        id.and_then(|id| self.get(id))
    }

    fn get_mut(&mut self, id: Option<Id<ID>>) -> Option<&mut T> {
        id.and_then(move |id| self.get_mut(id))
    }
}

impl<ID, T> Get1<&Option<Id<ID>>, T> for Comp1<ID, T> {
    fn get(&self, id: &Option<Id<ID>>) -> Option<&T> {
        id.and_then(|id| self.get(id))
    }

    fn get_mut(&mut self, id: &Option<Id<ID>>) -> Option<&mut T> {
        id.and_then(move |id| self.get_mut(id))
    }
}

// TODO Insert<Valid>

impl<ID, T> Comp1<ID, T> {
    fn get_index(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }

    fn get_mut_index(&mut self, index: usize) -> Option<&mut T> {
        self.values.get_mut(index)
    }

    fn insert_index(&mut self, index: usize, value: T) {
        debug_assert!(self.len() >= index);
        match self.len() {
            len if len > index => {
                if let Some(v) = self.values.get_mut(index) {
                    *v = value;
                }
            }
            len if len == index => self.values.push(value),
            _ => {}
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    pub fn zip_to_comp1<T2, F: Fn(&mut T, &T2)>(&mut self, rhs: &Comp1<ID, T2>, f: F) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(a, b)| f(a, b))
    }

    pub fn zip_to_comp1_by_2<T2, T3, F: Fn(&mut T, &T2, &T3)>(
        &mut self,
        a: &Comp1<ID, T2>,
        b: &Comp1<ID, T3>,
        f: F,
    ) {
        self.iter_mut()
            .zip(a.iter())
            .zip(b.iter())
            .for_each(|((a, b), c)| f(a, b, c));
    }

    pub fn zip_to_comp1_by_3<T2, T3, T4, F: Fn(&mut T, &T2, &T3, &T4)>(
        &mut self,
        a: &Comp1<ID, T2>,
        b: &Comp1<ID, T3>,
        c: &Comp1<ID, T4>,
        f: F,
    ) {
        self.iter_mut()
            .zip(a.iter())
            .zip(b.iter())
            .zip(c.iter())
            .for_each(|(((a, b), c), d)| f(a, b, c, d));
    }

    pub fn zip_to_comp1_by_4<T2, T3, T4, T5, F: Fn(&mut T, &T2, &T3, &T4, &T5)>(
        &mut self,
        a: &Comp1<ID, T2>,
        b: &Comp1<ID, T3>,
        c: &Comp1<ID, T4>,
        d: &Comp1<ID, T5>,
        f: F,
    ) {
        self.iter_mut()
            .zip(a.iter())
            .zip(b.iter())
            .zip(c.iter())
            .zip(d.iter())
            .for_each(|((((a, b), c), d), e)| f(a, b, c, d, e));
    }

    pub fn zip_to_comp2<T2, T3, F: Fn(&mut T, &T2, &T3)>(
        &mut self,
        rhs: &Comp2<ID, T2, T3>,
        f: F,
    ) {
        self.zip_to_comp1_by_2(&rhs.0, &rhs.1, f);
    }

    pub fn zip_to_comp1_and_comp2<T2, T3, T4, F: Fn(&mut T, &T2, &T3, &T4)>(
        &mut self,
        a: &Comp1<ID, T2>,
        b: &Comp2<ID, T3, T4>,
        f: F,
    ) {
        self.zip_to_comp1_by_3(a, &b.0, &b.1, f);
    }

    pub fn zip_to_comp2_and_comp2<T2, T3, T4, T5, F: Fn(&mut T, &T2, &T3, &T4, &T5)>(
        &mut self,
        a: &Comp2<ID, T2, T3>,
        b: &Comp2<ID, T4, T5>,
        f: F,
    ) {
        self.zip_to_comp1_by_4(&a.0, &a.1, &b.0, &b.1, f);
    }
}

impl<ID1, T: Copy> Comp1<ID1, T> {
    pub fn get_from<ID2>(&mut self, rhs: &Comp1<ID2, T>, ids: &Comp1<ID1, Id<ID2>>) {
        self.zip_to_comp1(ids, |value, id| {
            if let Some(v) = rhs.get(id) {
                *value = *v;
            }
        });
    }

    pub fn get_from_or<ID2>(&mut self, rhs: &Comp1<ID2, T>, ids: &Valid<ID1, ID2>, fallback: T) {
        self.values.iter_mut()
            .zip(ids.ids.iter())
            .for_each(|(v, id)|
                *v = id.and_then(|id| rhs.get(id))
                    .copied()
                    .unwrap_or(fallback)
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocators::FixedAllocator;

    #[test]
    fn zip_comp1_to_comp1() {
        let mut a = FixedAllocator::<()>::default();

        let mut t1 = Comp1::<(), u32>::default();
        let mut t2 = Comp1::<(), u32>::default();

        let id = a.create();
        t1.insert(id, 2);
        t2.insert(id, 3);

        t1.zip_to_comp1(&t2, |a, b| *a += *b);

        assert_eq!(5, t1.values[0]);
    }

    #[test]
    fn zip_comp1_to_comp1_and_comp1() {
        let mut a = FixedAllocator::<()>::default();

        let mut t1 = Comp1::<(), u32>::default();
        let mut t2 = Comp1::<(), u32>::default();
        let mut t3 = Comp1::<(), u32>::default();

        let id = a.create();
        t1.insert(id, 2);
        t2.insert(id, 3);
        t3.insert(id, 5);

        t1.zip_to_comp1_by_2(&t2, &t3, |a, b, c| *a += *b * *c);

        assert_eq!(17, t1.values[0]);
    }

    #[test]
    fn zip_comp1_to_comp2() {
        let mut a = FixedAllocator::<()>::default();

        let mut t1 = Comp1::<(), u32>::default();
        let mut t2 = Comp2::<(), u32, u32>::default();

        let id = a.create();
        t1.insert(id, 2);
        t2.insert(id, (3, 5));

        t1.zip_to_comp2(&t2,|a, b, c| *a += *b * *c);

        assert_eq!(17, t1.values[0]);
    }

    #[test]
    fn zip_to_comp2_and_comp1() {
        let mut a = FixedAllocator::<()>::default();

        let mut t1 = Comp1::<(), u32>::default();
        let mut t2 = Comp1::<(), u32>::default();
        let mut t3 = Comp2::<(), u32, u32>::default();

        let id = a.create();
        t1.insert(id, 2);
        t2.insert(id, 7);
        t3.insert(id, (3, 5));

        t1.zip_to_comp1_and_comp2(&t2, &t3, |a, b, c, d| *a += *b + (*c * *d));

        assert_eq!(24, t1.values[0]);
    }

    #[test]
    fn zip_to_comp2_and_comp2() {
        let mut a = FixedAllocator::<()>::default();

        let mut t1 = Comp1::<(), u32>::default();
        let mut t2 = Comp2::<(), u32, u32>::default();
        let mut t3 = Comp2::<(), u32, u32>::default();

        let id = a.create();
        t1.insert(id, 1);
        t2.insert(id, (2, 3));
        t3.insert(id, (5, 7));

        t1.zip_to_comp2_and_comp2(&t2, &t3, |a, b, c, d, e| *a += (*b * *c) + (*d * *e));

        assert_eq!(42, t1.values[0]);
    }

    #[derive(Default)] struct Type1;
    #[derive(Default)] struct Type2;

    #[test]
    fn get_from_id() {
        let mut taken = vec![];

        let mut alloc1 = FixedAllocator::<Type1>::default();
        let mut from_values = Comp1::<Type1, u8>::default();
        for i in 0..10 {
            let id = alloc1.create();
            from_values.insert(id, i);
            if i == 2 || i == 3 || i == 5 {
                taken.push(id);
            }
        }

        let mut alloc2 = FixedAllocator::<Type2>::default();
        let mut to_values = Comp1::<Type2, u8>::default();
        let mut ids = Comp1::<Type2, Id<Type1>>::default();

        for i in taken {
            let id = alloc2.create();
            to_values.insert(id, Default::default());
            ids.insert(id, i);
        }

        to_values.get_from(&from_values, &ids);

        assert_eq!(vec![2, 3, 5], to_values.values);
    }
}
