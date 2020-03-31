use crate::allocators::*;
use crate::{Insert, Get1};
use crate::components::Comp2;

#[derive(Debug, Default, Clone)]
pub struct GenIds<FROM, TO> {
    pub ids: Comp2<FROM, Option<Id<TO>>, Option<Gen>>,
    pub version: u64,
}

impl<FROM, TO, I: Indexes<FROM>> Insert<I, GenId<TO>> for GenIds<FROM, TO> {
    fn insert(&mut self, id: I, value: GenId<TO>) {
        self.ids.insert(id, Some(value));
    }
}

impl<FROM, TO, I: Indexes<FROM>> Insert<I, Option<GenId<TO>>> for GenIds<FROM, TO> {
    fn insert(&mut self, id: I, value: Option<GenId<TO>>) {
        self.ids.insert(id, value);
    }
}

impl<FROM, TO> GenIds<FROM, TO> {
    pub fn update(&mut self, alloc: &DynamicAllocator<TO>) {
        if alloc.version != self.version {
            self.remove_invalid_indices(&alloc);
            self.version = alloc.version;
        }
    }

    fn remove_invalid_indices(&mut self, alloc: &DynamicAllocator<TO>) {
        self.ids.0.iter_mut()
            .zip(self.ids.1.iter_mut())
            .for_each(|(id, gen)| {
                if !alloc.gen.get(*id)
                    .and_then(|current_gen| gen.map(|gen| *current_gen == gen))
                    .unwrap_or(false)
                {
                    *id = None;
                    *gen = None;
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Get2;

    #[derive(Debug, Default, Eq, PartialEq)] struct Type1;
    #[derive(Debug, Default)] struct Type2;

    #[test]
    fn update_test() {
        let mut dyn_alloc = DynamicAllocator::<Type1>::default();
        let mut fixed_alloc = FixedAllocator::<Type2>::default();

        let mut ids = GenIds::<Type2, Type1>::default();

        let dyn0 = dyn_alloc.create();
        let fixed0 = fixed_alloc.create();
        ids.insert(fixed0, dyn0);
        assert_eq!((&Some(dyn0.index), &Some(dyn0.gen)), ids.ids.get(fixed0).unwrap());

        assert_eq!(dyn_alloc.version, ids.version);
        ids.update(&dyn_alloc);

        let to_kill = dyn_alloc.create();
        dyn_alloc.kill(to_kill);

        assert_ne!(dyn_alloc.version, ids.version);
        ids.update(&dyn_alloc);
        assert_eq!(dyn_alloc.version, ids.version);

        dyn_alloc.kill(dyn0);
        assert_ne!(dyn_alloc.version, ids.version);
        ids.update(&dyn_alloc);
        assert_eq!(dyn_alloc.version, ids.version);
        assert_eq!((&None, &None), ids.ids.get(fixed0).unwrap());
    }
}