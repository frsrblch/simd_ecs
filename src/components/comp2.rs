use super::*;
use crate::allocators::{Indexes, Gen, Valid};

#[derive(Debug, Clone)]
pub struct Comp2<ID, T1, T2>(pub Comp1<ID, T1>, pub Comp1<ID, T2>);

impl<ID, T1, T2> Default for Comp2<ID, T1, T2> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<ID, T1, T2, I: Indexes<ID>> Insert<I, (T1, T2)> for Comp2<ID, T1, T2> {
    fn insert(&mut self, id: I, value: (T1, T2)) {
        self.0.insert(id, value.0);
        self.1.insert(id, value.1);
    }
}

impl<ID1, ID2, I: Indexes<ID1>> Insert<I, Option<GenId<ID2>>> for Comp2<ID1, Option<Id<ID2>>, Option<Gen>> {
    fn insert(&mut self, id: I, value: Option<GenId<ID2>>) {
        let t = value.map(|v| (Some(v.index), Some(v.gen))).unwrap_or((None, None));
        self.0.insert(id, t.0);
        self.1.insert(id, t.1);
    }
}

impl<ID, T1, T2> Get2<Id<ID>, T1, T2> for Comp2<ID, T1, T2> {
    fn get(&self, id: Id<ID>) -> Option<(&T1, &T2)> {
        self.0.get(id).and_then(|t1| self.1.get(id).map(|t2| (t1, t2)))
    }

    fn get_mut(&mut self, id: Id<ID>) -> Option<(&mut T1, &mut T2)> {
        let t1 = &mut self.0;
        let t2 = &mut self.1;
        t1.get_mut(id).and_then(move |t1| t2.get_mut(id).map(|t2| (t1, t2)))
    }
}

impl<ID, T1, T2> Get2<&Id<ID>, T1, T2> for Comp2<ID, T1, T2> {
    fn get(&self, id: &Id<ID>) -> Option<(&T1, &T2)> {
        self.get(*id)
    }

    fn get_mut(&mut self, id: &Id<ID>) -> Option<(&mut T1, &mut T2)> {
        self.get_mut(*id)
    }
}

impl<ID, T1, T2> Get2<Option<Id<ID>>, T1, T2> for Comp2<ID, T1, T2> {
    fn get(&self, id: Option<Id<ID>>) -> Option<(&T1, &T2)> {
        id.and_then(|id| self.get(id))
    }

    fn get_mut(&mut self, id: Option<Id<ID>>) -> Option<(&mut T1, &mut T2)> {
        id.and_then(move |id| self.get_mut(id))
    }
}

impl<ID, T1, T2> Get2<&Option<Id<ID>>, T1, T2> for Comp2<ID, T1, T2> {
    fn get(&self, id: &Option<Id<ID>>) -> Option<(&T1, &T2)> {
        id.and_then(|id| self.get(id))
    }

    fn get_mut(&mut self, id: &Option<Id<ID>>) -> Option<(&mut T1, &mut T2)> {
        id.and_then(move |id| self.get_mut(id))
    }
}

impl<ID, T1, T2> Comp2<ID, T1, T2> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T1, &T2)> {
        self.0.iter().zip(self.1.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut T1, &mut T2)> {
        self.0.iter_mut().zip(self.1.iter_mut())
    }
}

impl<ID, T1: Copy, T2: Copy> Comp2<ID, T1, T2> {
    pub fn get_from<ID2>(&mut self, rhs: &Comp2<ID2, T1, T2>, ids: Comp1<ID, Id<ID2>>) {
        self.0.iter_mut()
            .zip(ids.iter())
            .for_each(|(value, id)| {
                if let Some(v) = rhs.0.get(id) {
                    *value = *v;
                }
            });

        self.1.iter_mut()
            .zip(ids.iter())
            .for_each(|(value, id)| {
                if let Some(v) = rhs.1.get(id) {
                    *value = *v;
                }
            });
    }

    pub fn get_from_or<ID2>(&mut self, rhs: &Comp2<ID2, T1, T2>, ids: &Valid<ID, ID2>, fallback: (T1, T2)) {
        self.0.iter_mut()
            .zip(ids.ids.iter())
            .for_each(|(value, id)| {
                *value = id.and_then(|id| rhs.0.get(id))
                    .copied()
                    .unwrap_or(fallback.0)
            });

        self.1.iter_mut()
            .zip(ids.ids.iter())
            .for_each(|(value, id)| {
                *value = id.and_then(|id| rhs.1.get(id))
                    .copied()
                    .unwrap_or(fallback.1)
            });
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

        t1.iter_mut()
            .zip(t2.iter())
            .for_each(|((a, b), c)| {
                *a += *c;
                *b += *c;
            });

        assert_eq!(7, t1.0.values[0]);
        assert_eq!(9, t1.1.values[0]);
    }
}