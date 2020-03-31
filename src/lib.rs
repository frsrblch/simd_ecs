pub mod allocators;
pub mod components;
pub mod links;
pub mod prelude;

pub trait Insert<ID, T> {
    fn insert(&mut self, id: ID, value: T);
}

pub trait Get1<ID, T> {
    fn get(&self, id: ID) -> Option<&T>;
    fn get_mut(&mut self, id: ID) -> Option<&mut T>;
}

pub trait Get2<ID, T1, T2> {
    fn get(&self, id: ID) -> Option<(&T1, &T2)>;
    fn get_mut(&mut self, id: ID) -> Option<(&mut T1, &mut T2)>;
}