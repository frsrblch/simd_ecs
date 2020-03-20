use super::*;

#[derive(Debug, Default, Clone)]
pub struct Comp1<ID, T> {
    values: Vec<T>,
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
        match self.values.len() {
            len if len == index => self.values.push(value),
            len if len > index => {
                if let Some(v) = self.values.get_mut(index) {
                    *v = value;
                }
            }
            _ => {}
        }
    }

    fn iter(&self) -> impl Iterator<Item=&T> {
        self.values.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.values.iter_mut()
    }

    pub fn zip_to_comp1<T2, F: Fn(&mut T, &T2)>(&mut self, rhs: &Comp1<ID, T2>, f: F) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(a, b)| f(a, b))
    }

    pub fn zip_to_comp1_and_comp1<T2, T3, F: Fn(&mut T, &T2, &T3)>(&mut self, a: &Comp1<ID, T2>, b: &Comp1<ID, T3>, f: F) {
        self.iter_mut()
            .zip(a.iter())
            .zip(b.iter())
            .for_each(|((a, b), c)| f(a, b, c));
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
}
