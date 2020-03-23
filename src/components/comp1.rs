use super::*;

#[derive(Debug, Default, Clone)]
pub struct Comp1<ID, T> {
    pub values: Vec<T>,
    marker: PhantomData<ID>,
}

impl<ID, T> Insert<Id<ID>, T> for Comp1<ID, T> {
    fn insert(&mut self, id: Id<ID>, value: T) {
        self.insert_at_index(id.index, value);
    }
}

impl<ID, T> Insert<&Id<ID>, T> for Comp1<ID, T> {
    fn insert(&mut self, id: &Id<ID>, value: T) {
        self.insert_at_index(id.index, value);
    }
}

// TODO Insert<Valid>

impl<ID, T> Comp1<ID, T> {
    fn insert_at_index(&mut self, index: usize, value: T) {
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

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    pub fn zip_to_comp1<T2, F: Fn(&mut T, &T2)>(&mut self, rhs: &Comp1<ID, T2>, f: F) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(a, b)| f(a, b))
    }

    pub fn zip_to_comp1_and_comp1<T2, T3, F: Fn(&mut T, &T2, &T3)>(
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

    pub fn zip_to_comp2<T2, T3, F: Fn(&mut T, &T2, &T3)>(
        &mut self,
        rhs: &Comp2<ID, T2, T3>,
        f: F,
    ) {
        self.iter_mut()
            .zip(rhs.0.iter())
            .zip(rhs.1.iter())
            .for_each(|((a, b), c)| f(a, b, c))
    }

    pub fn zip_to_comp1_and_comp2<T2, T3, T4, F: Fn(&mut T, &T2, &T3, &T4)>(
        &mut self,
        a: &Comp1<ID, T2>,
        b: &Comp2<ID, T3, T4>,
        f: F,
    ) {
        self.iter_mut()
            .zip(a.values.iter())
            .zip(b.0.iter())
            .zip(b.1.iter())
            .for_each(|(((a, b), c), d)| f(a, b, c, d))
    }

    pub fn zip_to_comp2_and_comp2<T2, T3, T4, T5, F: Fn(&mut T, &T2, &T3, &T4, &T5)>(
        &mut self,
        a: &Comp2<ID, T2, T3>,
        b: &Comp2<ID, T4, T5>,
        f: F,
    ) {
        self.iter_mut()
            .zip(a.0.iter())
            .zip(a.1.iter())
            .zip(b.0.iter())
            .zip(b.1.iter())
            .for_each(|((((a, b), c), d), e)| f(a, b, c, d, e))
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

        t1.zip_to_comp1_and_comp1(&t2, &t3, |a, b, c| *a += *b * *c);

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
}
