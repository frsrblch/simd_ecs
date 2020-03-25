use super::*;

#[derive(Debug, Clone)]
pub struct Comp2<ID, T1, T2>(pub Vec<T1>, pub Vec<T2>, PhantomData<ID>);

impl<ID, T1, T2> Default for Comp2<ID, T1, T2> {
    fn default() -> Self {
        Self(Default::default(), Default::default(), PhantomData)
    }
}

impl<ID, T1, T2> Insert<Id<ID>, (T1, T2)> for Comp2<ID, T1, T2> {
    fn insert(&mut self, id: Id<ID>, value: (T1, T2)) {
        self.insert_at_index(id.index, value.0, value.1);
    }
}

impl<ID, T1, T2> Insert<&Id<ID>, (T1, T2)> for Comp2<ID, T1, T2> {
    fn insert(&mut self, id: &Id<ID>, value: (T1, T2)) {
        self.insert_at_index(id.index, value.0, value.1);
    }
}

impl<ID, T1, T2> Comp2<ID, T1, T2> {
    pub fn get(&self, id: Id<ID>) -> Option<(&T1, &T2)> {
        self.0.get(id.index)
            .and_then(|t1| self.1.get(id.index).map(|t2| (t1, t2)))
    }

    fn insert_at_index(&mut self, index: usize, a: T1, b: T2) {
        debug_assert!(self.len() >= index);
        match self.len() {
            len if len > index => {
                if let (Some(va), Some(vb)) = (self.0.get_mut(index), self.1.get_mut(index)) {
                    *va = a;
                    *vb = b;
                }
            }
            len if len == index => {
                self.0.push(a);
                self.1.push(b);
            }
            _ => {}
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn zip_to_comp1<T3, F: Fn(&mut T1, &mut T2, &T3)>(&mut self, rhs: &Comp1<ID, T3>, f: F) {
        self.0.iter_mut()
            .zip(self.1.iter_mut())
            .zip(rhs.values.iter())
            .for_each(|((a, b), c)| f(a, b, c))
    }

    pub fn zip_to_comp1_and_comp1<T3, T4, F: Fn(&mut T1, &mut T2, &T3, &T4)>(&mut self, a: &Comp1<ID, T3>, b: &Comp1<ID, T4>, f: F) {
        self.0.iter_mut()
            .zip(self.1.iter_mut())
            .zip(a.values.iter())
            .zip(b.values.iter())
            .for_each(|(((a, b), c), d)| f(a, b, c, d));
    }

    pub fn zip_to_comp2<T3, T4, F: Fn(&mut T1, &mut T2, &T3, &T4)>(&mut self, rhs: &Comp2<ID, T3, T4>, f: F) {
        self.0.iter_mut()
            .zip(self.1.iter_mut())
            .zip(rhs.0.iter())
            .zip(rhs.1.iter())
            .for_each(|(((a, b), c), d)| {
                f(a, b, c, d);
            })
    }

    pub fn zip_to_comp1_and_comp2<T3, T4, T5, F: Fn(&mut T1, &mut T2, &T3, &T4, &T5)>(&mut self, comp1: &Comp1<ID, T3>, comp2: &Comp2<ID, T4, T5>, f: F) {
        self.0.iter_mut()
            .zip(self.1.iter_mut())
            .zip(comp1.values.iter())
            .zip(comp2.0.iter())
            .zip(comp2.1.iter())
            .for_each(|((((a, b), c), d), e)| {
                f(a, b, c, d, e);
            })
    }
}

impl<ID, T1> Comp2<ID, T1, T1> {
    pub fn zip_each_to_comp2<T2, F: Fn(&mut T1, &T2)>(&mut self, rhs: &Comp2<ID, T2, T2,>, f: F) {
        self.0.iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(a, b)| f(a, b));

        self.1.iter_mut()
            .zip(rhs.1.iter())
            .for_each(|(a, b)| f(a, b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocators::FixedAllocator;

    #[test]
    fn zip_to_comp1() {
        let mut a = FixedAllocator::<()>::default();

        let mut t1 = Comp2::<(), u32, u32>::default();
        let mut t2 = Comp1::<(), u32>::default();

        let id = a.create();
        t1.insert(id, (5, 7));
        t2.insert(id, 2);

        t1.zip_to_comp1(&t2,|a, b, c| {
            *a += *c;
            *b += *c;
        });

        assert_eq!(7, t1.0[0]);
        assert_eq!(9, t1.1[0]);
    }
}